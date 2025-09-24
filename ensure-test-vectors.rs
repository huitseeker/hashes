//! Standalone test vector setup program for BLAKE2X tests
//!
//! This program downloads and extracts BLAKE2X test vectors from the official BLAKE2 repository.
//! It should be run as a build script or test setup utility.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("Ensuring BLAKE2X test vectors are available...");

    // Check if we have all required test vector files
    let blake2xb_path = Path::new("blake2/tests/data/blake2xb/blake2xb-kat.json");
    let blake2xs_path = Path::new("blake2/tests/data/blake2xs/blake2xs-kat.json");

    if blake2xb_path.exists() && blake2xs_path.exists() {
        println!("Test vector files already exist. Skipping download.");
        return;
    }

    // Create cache directory
    let cache_dir = Path::new(".cache");
    fs::create_dir_all(cache_dir).expect("Failed to create cache directory");

    let source_path = cache_dir.join("blake2-kat.json");

    // Download the source file if it doesn't exist
    if !source_path.exists() {
        println!("Downloading BLAKE2 test vectors...");
        let output = Command::new("curl")
            .args(&[
                "-s",
                "-o",
                source_path.to_str().unwrap(),
                "https://raw.githubusercontent.com/BLAKE2/BLAKE2/master/testvectors/blake2-kat.json"
            ])
            .output()
            .expect("Failed to download test vectors");

        if !output.status.success() {
            panic!("Failed to download test vectors: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Verify the download worked by checking file size
        let metadata = fs::metadata(&source_path).expect("Failed to get downloaded file metadata");
        if metadata.len() == 0 {
            panic!("Downloaded file is empty");
        }
    }

    // Extract BLAKE2XB test vectors
    if !blake2xb_path.exists() {
        println!("Extracting BLAKE2XB test vectors...");
        let output = Command::new("jq")
            .args(&[
                "[.[] | select(.hash == \"blake2xb\")]",
                source_path.to_str().unwrap(),
                "-c"
            ])
            .output()
            .expect("Failed to extract BLAKE2XB test vectors");

        if !output.status.success() {
            panic!("Failed to extract BLAKE2XB test vectors: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Ensure parent directory exists
        fs::create_dir_all(blake2xb_path.parent().unwrap()).expect("Failed to create test data directory");

        fs::write(&blake2xb_path, output.stdout)
            .expect("Failed to write BLAKE2XB test vectors");
    }

    // Extract BLAKE2XS test vectors
    if !blake2xs_path.exists() {
        println!("Extracting BLAKE2XS test vectors...");
        let output = Command::new("jq")
            .args(&[
                "[.[] | select(.hash == \"blake2xs\")]",
                source_path.to_str().unwrap(),
                "-c"
            ])
            .output()
            .expect("Failed to extract BLAKE2XS test vectors");

        if !output.status.success() {
            panic!("Failed to extract BLAKE2XS test vectors: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Ensure parent directory exists
        fs::create_dir_all(blake2xs_path.parent().unwrap()).expect("Failed to create test data directory");

        fs::write(&blake2xs_path, output.stdout)
            .expect("Failed to write BLAKE2XS test vectors");
    }

    println!("Test vectors successfully prepared!");
}