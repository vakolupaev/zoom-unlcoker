TARGET=armv7-unknown-linux-gnueabihf
BINARY=zoom-unlocker

build:
	cross build --release --target=$(TARGET) && scp target/$(TARGET)/release/$(BINARY) root@192.168.1.10:/usr/bin

build-only:
	cross build --release --target=$(TARGET)

upload:
	scp target/$(TARGET)/release/$(BINARY) root@192.168.1.10:/usr/bin

clean:
	cargo clean
