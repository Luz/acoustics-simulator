run: all
	cargo run --release
output.mp4: run
	ffmpeg -framerate 60 -start_number 0 -i ./output/output-%04d.png ./output.mp4
clean:
	cargo clean
all: src/main.rs
	mkdir -p output
	cargo build --release
