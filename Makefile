BUILD_FRAVER := release
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG := --release

export BUILD_FRAVER
export RUSTC_TERGET
export CARGO_FLAG

include bootloader/Makefile
include kernel/Makefile

OBJDIR := bin/obj

bin/obj/bootloader: bootloader/target/i386-unknown-none/release/bootloader
	objcopy -O binary -S target/i386-unknown-none/release/rabbit bin/obj/bootloader

bin/obj/kernel: kernel/target/i386-unknown-none/release/kernel
	objcopy -O binary -S target/i386-unknown-none/release/rabbit bin/obj/kernel

bin/rabbit.img: bin/obj/kernel bin/obj/bootloader
	dd if=/dev/zero of=bin/img/rabbit.img~ count=10000 2>/dev/null
	dd if=bin/obj/bootloader of=bin/img/rabbit.img~ conv=notrunc 2>/dev/null
	dd if=bin/obj/kernel of=bin/img/rabbit.img~ seek=1 conv=notrunc 2>/dev/null
	mv bin/img/rabbit.img~ bin/rabbit.img

.PHONY: objcopy
objcopy:
	docker run -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit objcopy -O binary -S target/i386-unknown-none/release/rabbit bin/rabbit.bin

.PHONY: login
login:
	docker-compose run --rm rabbit /bin/bash

.PHONY: run
run:
	qemu-system-i386 bin/rabbit.bin
