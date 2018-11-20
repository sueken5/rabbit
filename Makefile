BUILD_FRAVER := release
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG := --release

.PHONY: build
build:
	# cd bootloader && xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)
	cd kernel && xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)
	mkdir -p bin/obj
	objcopy -O binary bootloader/target/i386-unknown-none/release/bootloader bin/obj/bootloader
	objcopy -O binary kernel/target/i386-unknown-none/release/kernel bin/obj/kernel
	mkdir -p bin/img
	dd if=/dev/zero of=bin/img/rabbit.img~ count=10000 2>/dev/null
	dd if=bin/obj/bootloader of=bin/img/rabbit.img~ conv=notrunc 2>/dev/null
	dd if=bin/obj/kernel of=bin/img/rabbit.img~ seek=1 conv=notrunc 2>/dev/null
	mv bin/img/rabbit.img~ bin/rabbit.img

.PHONY: login
login:
	docker-compose run --rm rabbit /bin/bash

.PHONY: run
run:
	qemu-system-i386 bin/rabbit.img
