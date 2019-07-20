.DEFAULT_GOAL := make

make:
	RUST_TARGET_PATH="/home/jonah/Projects/smart-plug/" RUSTUP_TOOLCHAIN=avr-toolchain XARGO_RUST_SRC="/home/jonah/Projects/rust/src" xargo build --target avr-atmega328p --release

install: make
	avr-objcopy -S -j .text -j .data -O ihex target/avr-atmega328p/release/smart-plug.elf target/avr-atmega328p/release/smart-plug.hex
	sudo avrdude -p atmega328p -c arduino -P /dev/ttyACM0 -U flash:w:target/avr-atmega328p/release/smart-plug.hex:i

