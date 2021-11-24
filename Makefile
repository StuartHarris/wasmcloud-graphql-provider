.PHONY:default
default: build

.PHONY:clean
clean:
	cd interface && $(MAKE) clean
	cd actor && $(MAKE) clean
	cd provider && $(MAKE) clean

.PHONY:build
build:
	cd interface && $(MAKE) build
	cd actor && $(MAKE) build
	cd provider && $(MAKE) build

.PHONY:push
push:
	cd interface && $(MAKE) push
	cd actor && $(MAKE) push
	cd provider && $(MAKE) push
