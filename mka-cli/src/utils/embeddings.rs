use anyhow::Result;
use ort::session::Session;
use ort::value::Value;
use ort::session::builder::GraphOptimizationLevel;
use tokenizers::Tokenizer;
use crate::models::configs::Config;

pub struct Embedder {
    session: Session,
    tokenizer: Tokenizer,
}

impl Embedder {
    pub fn new() -> Result<Self> {
        let model_path = Config::get_model_path()?;
        let tokenizer_path = Config::get_tokenizer_path()?;

        if !model_path.exists() || !tokenizer_path.exists() {
            return Err(anyhow::anyhow!("Model or tokenizer not found. Run 'mka model-install' first."));
        }

        let session = Session::builder()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .with_intra_threads(4)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .commit_from_file(model_path)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

        Ok(Self {
            session,
            tokenizer,
        })
    }

    pub fn generate_embedding(&mut self, text: &str) -> Result<Vec<f32>> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&x| x as i64).collect();
        let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&x| x as i64).collect();
        let token_type_ids: Vec<i64> = encoding.get_type_ids().iter().map(|&x| x as i64).collect();

        let length = input_ids.len();
        
        // In ort v2.0.0-rc.12, from_array takes a single argument that implements OwnedTensorArrayData
        // We use the (shape, data) tuple format. Shape must be a Vec<usize> or similar.
        let input_ids_tensor = Value::from_array((vec![1, length], input_ids))
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let attention_mask_tensor = Value::from_array((vec![1, length], attention_mask.clone()))
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let token_type_ids_tensor = Value::from_array((vec![1, length], token_type_ids))
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let inputs = ort::inputs![
            "input_ids" => input_ids_tensor,
            "attention_mask" => attention_mask_tensor,
            "token_type_ids" => token_type_ids_tensor,
        ];

        let outputs = self.session.run(inputs)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let (shape, data) = outputs["last_hidden_state"].try_extract_tensor::<f32>()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        
        // shape is [1, seq_len, 384]
        let seq_len = shape[1] as usize;
        let dim = shape[2] as usize;

        // Mean Pooling
        let mut pooled = vec![0.0f32; dim];
        let mut count = 0.0f32;

        for i in 0..seq_len {
            if attention_mask[i] == 1 {
                for j in 0..dim {
                    pooled[j] += data[i * dim + j];
                }
                count += 1.0;
            }
        }

        if count > 0.0 {
            for j in 0..dim {
                pooled[j] /= count;
            }
        }

        // L2 Normalization
        let mut norm = 0.0f32;
        for &val in &pooled {
            norm += val * val;
        }
        norm = norm.sqrt();

        if norm > 0.0 {
            for val in &mut pooled {
                *val /= norm;
            }
        }

        Ok(pooled)
    }
}
