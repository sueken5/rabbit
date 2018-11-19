include bootloader/Makefile
# include kernel/Makefile


# .PHONY: kernel
# kernel:
# 	docker run -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit xargo build --target i386-unknown-none --release
#
# .PHONY: run
# run:
# 	qemu-system-i386 bin/rabbit.bin
#
# .PHONY: objcopy
# objcopy:
# 	docker run -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit objcopy -O binary -S target/i386-unknown-none/release/rabbit bin/rabbit.bin

.PHONY: login
login:
	docker run -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit /bin/bash
