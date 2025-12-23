.PHONY: all check release

NPROCS := $(shell nproc)

all: target/debug/testcgi

check:
	cargo check

clean:
	cargo clean --verbose
	rm Cargo.lock

release: target/release/testcgi

target/debug/testcgi: src/*.rs
	cargo build --profile dev --jobs $(NPROCS)

target/release/testcgi: src/*.rs
	cargo build --profile release --jobs $(NPROCS)
