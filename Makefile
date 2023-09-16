# test:
# @cargo nextest run

build-fc-prod:
	@rm -rf aliyun-fc && mkdir aliyun-fc
	@TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/readbot aliyun-fc && cp config.prod.yaml aliyun-fc

build-fc-test:
	@rm -rf aliyun-fc && mkdir aliyun-fc
	@TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/readbot aliyun-fc && cp config.test.yaml aliyun-fc
