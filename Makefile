.PHONY: kernel
kernel:
	docker run -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit bootimage build --target x86_64-rabbit.json

.PHONY: run
run:
	qemu-system-x86_64 target/x86_64-rabbit/debug/bootimage-rabbit.bin

.PHONY: docker-login
docker-login:
	docker run  -v $(shell pwd):/rabbit -it -w=/rabbit --rm sueken5/rabbit /bin/bash

.PHONY: docker-build
docker-build:
	docker build -t sueken5/rabbit ./
