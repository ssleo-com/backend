cargo build --release;
sudo cp ./target/release/backend /usr/local/bin/backend;
cd migrations;
cargo build --release;
sudo cp ./target/release/migrations /usr/local/bin/migrate;
cd -;