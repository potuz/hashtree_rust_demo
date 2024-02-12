extern crate hashtree_rust_demo;
mod merkleize;

// use merkleize::sparse_hash_tree;
use std::time::Instant;

fn main() {
    // Initialize the library
    let cpu_detected = hashtree_rust_demo::init();
    if cpu_detected == 0 {
        println!("hashtree initialization failed");
        return;
    }

    let chunks = [0u8; 1 << 22];

    let start = Instant::now();
    let hash_tree = merkleize::sparse_hashtree(&chunks, 0);
    let duration = start.elapsed();

    println!("Hashed data: {:?}", &hash_tree[hash_tree.len() - 32..]);
    println!("Zero hash: {:?}", &merkleize::ZERO_HASH_ARRAY[17][..]);
    println!("Time elapsed: {:?}", duration.as_micros());
}
