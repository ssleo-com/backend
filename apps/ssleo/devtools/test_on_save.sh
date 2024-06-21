while inotifywait -r -e modify,create,delete src 2>/dev/null; do
	clear
	cargo test -- --nocapture
done
