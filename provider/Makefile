CAPABILITY_ID = stuart-harris:graphql-provider
NAME = wasmcloud-graphql-provider
VENDOR = StuartHarris
PROJECT = wasmcloud-graphql-provider
VERSION = 0.1.0
REVISION = 0
ARCHITECTURE = x86_64-linux

.PHONY: default
default:
	$(MAKE) build
	$(MAKE) par

.PHONY: build
build:
	yarn && yarn build
	yarn --production
	mkdir -p build
	tar -czf build/build.tgz node_modules dist
	cargo build --release

.PHONY: clean
clean:
	rm -rf ./dist ./build
	cargo clean

.PHONY: par
par:
	wash par create \
		--arch $(ARCHITECTURE) \
		--binary target/release/$(PROJECT) \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination build/$(PROJECT).par.gz \
		--compress
	wash par inspect build/$(PROJECT).par.gz

.PHONY: push
push:
	wash reg push --insecure registry:5001/$(PROJECT):$(VERSION) build/$(PROJECT).par.gz
