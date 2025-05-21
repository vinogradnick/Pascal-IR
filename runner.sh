cargo check
rust_bundler_cp --input . >outpost.rs

cargo watch -s 'cat in.txt | cargo run' #watcher
