SRCS := $(shell find src -name *.rs)

output.mp4: output/output-0000.png
	ffmpeg -framerate 60 -start_number 0 -i ./output/output-%04d.png ./output.mp4 -y
output/output-0000.png: target/release/acoustics-simulator
	cargo run --release
target/release/acoustics-simulator: $(SRCS)
	mkdir -p output
	cargo build --release
clean:
	cargo clean
	rm -f output/output-????.png
