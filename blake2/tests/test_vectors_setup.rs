//! Test vector setup integration test
//!
//! This test ensures that test vectors are available before running BLAKE2X tests.
//! It runs the standalone test vector setup program if needed.

#![cfg(feature = "blake2x")]

#[test]
fn ensure_test_vectors_available() {
    use std::process::Command;
    use std::path::Path;

    // Check if test vector files exist
    let blake2xb_path = Path::new("tests/data/blake2xb/blake2xb-kat.json");
    let blake2xs_path = Path::new("tests/data/blake2xs/blake2xs-kat.json");

    // If both files exist, we're good
    if blake2xb_path.exists() && blake2xs_path.exists() {
        println!("Test vector files already exist");
        return;
    }

    // Run the setup program
    println!("Running test vector setup...");
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "ensure-test-vectors",
            "--manifest-path",
            "../Cargo.toml",
            "--quiet"
        ])
        .output()
        .expect("Failed to run test vector setup");

    if !output.status.success() {
        println!("Test vector setup failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Test vector setup failed");
    }

    // Verify files were created
    assert!(
        blake2xb_path.exists(),
        "BLAKE2XB test vectors should have been created"
    );
    assert!(
        blake2xs_path.exists(),
        "BLAKE2XS test vectors should have been created"
    );

    println!("Test vectors successfully prepared");
}