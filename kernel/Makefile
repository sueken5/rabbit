#
# Makefile fragment for the JOS kernel.
# This is NOT a complete makefile;
# you must run GNU make in the top-level directory
# where the GNUmakefile is located.
#

kernel/target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/kernel: kernel/src/*.rs kernel/src/*.S kernel/$(RUSTC_TERGET).json
	cd kernel; xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)
