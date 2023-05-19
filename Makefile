# test:
# @cargo nextest run

build-fc:
	@TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/openai-http-proxy aliyun-fc
