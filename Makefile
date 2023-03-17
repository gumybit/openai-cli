release-file:
	cargo build --release
	tar -C target/release -czf openai-cli.mac.tar.gz openai-cli