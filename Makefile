test:
	cargo test --manifest-path ./life/Cargo.toml

play: 
	cargo run --manifest-path ./life/Cargo.toml --release

clean:
	cargo clean --manifest-path ./life/Cargo.toml

build:
	cargo build --manifest-path ./life/Cargo.toml

run: 
	cargo run --manifest-path ./life/Cargo.toml
