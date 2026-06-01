#[cfg(test)]
mod tests {
    use crate::utils::embeddings::Embedder;
    use crate::models::configs::Config;

    #[test]
    fn test_generate_embedding() {
        // Skip if model is not installed
        let model_path = Config::get_model_path().unwrap();
        if !model_path.exists() {
            println!("Skipping embedding test as model is not installed.");
            return;
        }

        let mut embedder = Embedder::new().unwrap();
        let text = "Initialize MKA by sparse-cloning the template structure from GitHub.";
        let embedding = embedder.generate_embedding(text).unwrap();

        assert_eq!(embedding.len(), 384);
        
        // Check for L2 normalization (sum of squares should be approx 1)
        let norm: f32 = embedding.iter().map(|&x| x * x).sum();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_embedding_similarity() {
        let model_path = Config::get_model_path().unwrap();
        if !model_path.exists() {
            return;
        }

        let mut embedder = Embedder::new().unwrap();
        
        let text1 = "start mcp server";
        let text2 = "run mcp daemon";
        let text3 = "install python parser";

        let emb1 = embedder.generate_embedding(text1).unwrap();
        let emb2 = embedder.generate_embedding(text2).unwrap();
        let emb3 = embedder.generate_embedding(text3).unwrap();

        let dot_product = |a: &[f32], b: &[f32]| -> f32 {
            a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
        };

        let sim12 = dot_product(&emb1, &emb2);
        let sim13 = dot_product(&emb1, &emb3);

        println!("Similarity (1,2): {}, Similarity (1,3): {}", sim12, sim13);
        assert!(sim12 > sim13, "Semantic similar texts should have higher similarity score");
    }
}
