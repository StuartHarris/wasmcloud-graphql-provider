CAPABILITY_ID = "stuart-harris:graphql-provider"
NAME = "wasmcloud-graphql-provider"
VENDOR = "StuartHarris"
PROJECT = wasmcloud-graphql-provider
VERSION = 0.1.0
REVISION = 0

.PHONY: default
default:
	yarn && yarn build
	yarn --production
	mkdir -p build
	tar -czf build/build.tgz node_modules dist
	cargo run

.PHONY: clean
clean:
	rm -rf ./dist ./build
	cargo clean

.PHONY: build
build:
	yarn && yarn build
	yarn --production
	mkdir -p build
	tar -czf build/build.tgz node_modules dist
	cargo build --release
	wash par create \
		--arch x86_64-macos \
		--binary target/release/$(PROJECT) \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination build/$(PROJECT).par.gz \
		--compress
	wash par inspect build/$(PROJECT).par.gz
