build:
	@cargo build --release
	@mv target/release/hack_assembler ./hack
	@chmod +x ./hack