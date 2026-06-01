use anyhow::{Result, anyhow};
use crate::models::{MkaIndex, configs::Config};
use crate::utils::database::Database;
use crate::utils::embeddings::Embedder;
use crate::commands::model_install;
use dialoguer::Confirm;
use sha2::{Sha256, Digest};

pub async fn handle(query: String) -> Result<()> {
    // Check if model exists, prompt if not (for CLI usage only)
    let model_path = Config::get_model_path()?;
    if !model_path.exists() {
        if Confirm::new()
            .with_prompt("Semantic search model not found. Would you like to download it now? (approx. 80MB)")
            .default(true)
            .interact()?
        {
            model_install::handle().await?;
        } else {
            return Err(anyhow!("Model required for semantic search."));
        }
    }

    let output = get_search_results(&query).await?;
    println!("{}", output);
    Ok(())
}

pub async fn get_search_results(query: &str) -> Result<String> {
    // 1. Check if model exists
    let model_path = Config::get_model_path()?;
    if !model_path.exists() {
        return Err(anyhow!("Semantic search model not found. Run 'mka model-install' first."));
    }

    // Load index
    let index_path = Config::get_index_file()?;
    if !index_path.exists() {
        return Err(anyhow!("index.mka.yaml not found."));
    }
    let content = tokio::fs::read_to_string(index_path).await?;
    let index: MkaIndex = serde_yaml::from_str(&content)?;

    // 2. Open DB and Embedder
    let db = Database::open()?;
    let mut embedder = Embedder::new()?;

    // 3. Sync Index with DB
    sync_index_with_db(&db, &mut embedder, &index)?;

    // 4. Generate query embedding
    let query_embedding = embedder.generate_embedding(query)?;

    // 5. Search
    let raw_results = db.search(&query_embedding, 5)?;

    if raw_results.is_empty() {
        return Ok("No relevant workflows found.".to_string());
    }

    // 6. Calculate Similarity with Keyword Boosting
    let query_words: Vec<String> = query.to_lowercase()
        .split_whitespace()
        .map(|w| w.chars().filter(|c| c.is_alphanumeric()).collect())
        .filter(|s: &String| !s.is_empty())
        .collect();

    let mut results = Vec::new();
    for (id, d) in raw_results {
        let mut similarity = 1.0 - (d * d / 2.0);
        
        // Find the intent in the index to check for keyword matches
        if let Some(workflow) = index.workflows.iter().find(|w| w.id == id) {
            let intent_lower = workflow.intent.to_lowercase();
            let mut match_count = 0;
            for word in &query_words {
                if intent_lower.contains(word) {
                    match_count += 1;
                }
            }
            // Boost for matching words (up to 0.3)
            if !query_words.is_empty() {
                let boost = (match_count as f32 / query_words.len() as f32) * 0.3;
                similarity += boost;
            }
        }
        results.push((id, similarity));
    }

    // Sort by boosted similarity
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let top_score = results[0].1;
    let best_id = &results[0].0;

    // 7. Auto-fetch logic:
    // We auto-fetch if:
    // - Top score is very high (> 0.6)
    // - OR Top score is positive (> 0.3) AND there's a clear gap (> 0.15)
    
    let should_auto_fetch = if results.len() == 1 {
        top_score > 0.3
    } else {
        let second_score = results[1].1;
        let gap = top_score - second_score;
        top_score > 0.6 || (top_score > 0.3 && gap > 0.15)
    };

    if should_auto_fetch {
        drop(db);
        let content = crate::commands::workflow_get::get_workflow_content(best_id, false).await?;
        return Ok(format!("@mka:search_result_perfect_match\nNote: Top match '{}' is highly relevant (score: {:.4}). Returning full details directly.\n\n{}", best_id, top_score, content));
    }

    // 8. Otherwise output the list in TOON format
    let mut output = String::from("@mka:search_results\n");
    for (id, score) in results {
        output.push_str(&format!("- [{}]: (score: {:.4})\n", id, score));
    }

    Ok(output)
}

fn sync_index_with_db(db: &Database, embedder: &mut Embedder, index: &MkaIndex) -> Result<()> {
    let mut active_ids = Vec::new();

    for workflow in &index.workflows {
        active_ids.push(workflow.id.clone());
        
        let mut hasher = Sha256::new();
        hasher.update(workflow.intent.as_bytes());
        let current_hash = format!("{:x}", hasher.finalize());

        let stored_hash = db.get_intent_hash(&workflow.id)?;

        if stored_hash.as_deref() != Some(&current_hash) {
            let embedding = embedder.generate_embedding(&workflow.intent)?;
            db.upsert_workflow(&workflow.id, &current_hash, &embedding)?;
        }
    }

    db.cleanup_stale_workflows(&active_ids)?;

    Ok(())
}
