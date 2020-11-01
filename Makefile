.PHONY: watch
watch:
	cargo watch -c -d 0.5 -x 'test -- --nocapture'
