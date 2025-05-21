check:
	cargo check
# Генерация outpost.rs
bundle:
	rust_bundler_cp --input . > outpost.rs

# Запуск cargo-watch с чтением in.txt и передачей в cargo run
watch:
	cargo watch -s "cargo run < in.txt "

# Удобная цель: сначала собрать, потом запустить вотчер
build: check bundle
