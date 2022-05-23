all: build
	cargo run --release

art: build
	cargo run --release art

build:
	cargo build --release

clean:
	cargo clean