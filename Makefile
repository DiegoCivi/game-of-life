test:
	cargo test --manifest-path ./life/Cargo.toml

play: 
	cargo run --manifest-path ./life/Cargo.toml

clean:
	cargo clean --manifest-path ./life/Cargo.toml

build:
	cargo build --manifest-path ./life/Cargo.toml
