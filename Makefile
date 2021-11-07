.PHONY: build
build:
	cd interface && $(MAKE)
	cd actor && $(MAKE)
	cd provider && $(MAKE)

.PHONY: clean
clean:
	cd interface && $(MAKE) clean
	cd actor && $(MAKE) clean
	cd provider && $(MAKE) clean
