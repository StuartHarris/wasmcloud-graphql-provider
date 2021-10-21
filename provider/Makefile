CAPABILITY_ID = "stuart-harris:graphql-provider"
NAME = "wasmcloud-graphql-provider"
VENDOR = "StuartHarris"
PROJECT = wasmcloud-graphql-provider
VERSION = 0.1.0
REVISION = 0

.PHONY: default
default: clean
	yarn && yarn build
	cargo run

.PHONY: clean
clean:
	rm -rf ./dist

.PHONY: build
build:
	yarn && yarn build
	cargo build --release
	wash par create \
		--arch aarch64-linux \
		--binary target/release/$(PROJECT) \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination build/$(PROJECT).par.gz \
		--compress
	wash par inspect build/$(PROJECT).par.gz
