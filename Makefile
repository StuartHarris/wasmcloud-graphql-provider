.PHONY: default
default: clean
	yarn && yarn build
	cargo run

.PHONY: clean
clean:
	rm -rf ./dist
