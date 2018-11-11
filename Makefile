.PHONY: kernel
kernel:
	docker run -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit xargo build --target x86_64-rabbit --release

.PHONY: run
run:
	qemu-system-x86_64 target/x86_64-rabbit/release/rabbit
