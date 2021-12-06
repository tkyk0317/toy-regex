.PHONY: all
all: build clippy test bench down

.PHONY: build
build:
	docker-compose build
	docker-compose up -d
	docker exec -t toy-regex-rust cargo b --verbose

.PHONY: clippy
clippy: build
	docker exec -t toy-regex-rust cargo clippy

.PHONY: test
test: build
	docker exec -t toy-regex-rust cargo t

.PHONY: bench
bench: build
	docker exec -t toy-regex-rust cargo +nightly bench

.PHONY: down
down:
	docker-compose down

profile: build
	docker exec -t toy-regex-rust cargo b --release
	docker exec -t toy-regex-rust cargo profiler callgrind --bin ./target/release/toy_regex -- -vm -s aaaaaaaaaaaaaaaaaaaa .*a -n 10

