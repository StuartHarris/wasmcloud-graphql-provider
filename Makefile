.PHONY: clean-build
clean-build:
	cd interface && $(MAKE) clean && $(MAKE)
	cd actor && $(MAKE) clean && $(MAKE)
	cd provider && $(MAKE) clean && $(MAKE) build
