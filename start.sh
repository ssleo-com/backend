cd models;
cargo build --release;
cd ../;
cd shared;
cargo build --release;
cd ../;
cd controllers;
cargo build --release;
cd ../;
cd views;
cargo build --release;
cd ../;
./views/target/release/backend