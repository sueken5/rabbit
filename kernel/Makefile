BUILD_FRAVER := release
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG :=
ifeq ($(BUILD_FRAVER),release)
	CARGO_FLAG += --release
endif

target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/kernel: src/*.rs src/*.S $(RUSTC_TERGET).json
	xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)
