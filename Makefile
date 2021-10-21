.PHONY: default
default: clean
	yarn && yarn build
	cargo run

.PHONY: clean
clean:
	rm -rf ./dist

.PHONY: server
server: clean
	yarn && yarn start
