.PHONY: build clean run

build:
	@echo "Compiling the project..."
	@cargo build

release:
	@echo "Creating release build..."
	@cargo build --release

clean:
	@echo "Cleaning up..."
	@cargo clean
	@rm -f ./ancestry

run:
	@echo "Running the project..."
	@RUST_BACKTRACE=full cargo run

test:
	@echo "Running tests..."
	@cargo test

binary: release
	@echo "Creating binary..."
	@cp ./target/release/ancestry ./ancestry
	@chmod +x ./ancestry