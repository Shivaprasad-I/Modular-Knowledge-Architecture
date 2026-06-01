use anyhow::{Result, Context};
use crate::models::configs::Config;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn handle() -> Result<()> {
    let model_path = Config::get_model_path()?;
    let tokenizer_path = Config::get_tokenizer_path()?;

    println!("Installing semantic search model...");

    download_file(
        "https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx",
        &model_path,
        "Downloading model (80MB)"
    ).await?;

    download_file(
        "https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/tokenizer.json",
        &tokenizer_path,
        "Downloading tokenizer"
    ).await?;

    println!("Model installed successfully to {}", Config::get_app_data_dir()?.display());
    Ok(())
}

pub async fn download_file(url: &str, path: &std::path::Path, msg: &str) -> Result<()> {
    if path.exists() {
        println!("{} already exists.", msg);
        return Ok(());
    }

    let response = reqwest::get(url).await?;
    let total_size = response
        .content_length()
        .context(format!("Failed to get content length from {}", url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));
    pb.set_message(msg.to_string());

    let mut file = File::create(path).await?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        let new = std::cmp::min(pb.position() + (chunk.len() as u64), total_size);
        pb.set_position(new);
    }

    pb.finish_with_message(format!("{} completed", msg));
    Ok(())
}
