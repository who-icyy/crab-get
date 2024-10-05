use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub fn download_file(url: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Send the request and get the response
    let mut response = client.get(url).send()?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }

    // Get the content length, if available
    let total_size = response.content_length().unwrap_or(0);

    // Create a progress bar
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_length(30);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.red}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Create the file to write to
    let mut file = File::create(output)?;

    // Download the file in chunks and update the progress bar
    let mut downloaded: u64 = 0;
    let mut buffer = vec![0; 8192]; // 8 KB buffer for downloading in chunks
    while let Ok(bytes_read) = response.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        // Update the progress bar
        progress_bar.set_position(downloaded);
    }

    // Finish the progress bar
    progress_bar.finish_with_message("Download complete");

    Ok(())
}