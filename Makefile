.PHONY: watch build install
watch:
	cargo watch -c -d 0.5 -x 'test -- --nocapture'
install:
	cargo install cargo-watch
build:
	cargo build
