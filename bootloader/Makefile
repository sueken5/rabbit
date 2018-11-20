#
# Makefile fragment for the JOS kernel.
# This is NOT a complete makefile;
# you must run GNU make in the top-level directory
# where the GNUmakefile is located.
#

bootloader/target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader: bootloader/src/*.rs bootloader/src/*.S bootloader/$(RUSTC_TERGET).json
	cd bootloader; xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)
