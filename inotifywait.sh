while inotifywait -r -e modify,create,delete src 2>/dev/null; do
	cargo fmt
	cargo build --release
done
