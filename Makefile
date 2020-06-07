make:
	@cargo build
lint:
	@cargo fmt
serve:
	python3 -m http.server 8080