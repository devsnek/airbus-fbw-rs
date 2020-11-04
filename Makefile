SOURCES := $(wildcard ./fbw/**/*) $(wildcard ./model/**/*)
BUILD ?= debug
CARGO_FLAGS = --target wasm32-wasi

all: target/fbw-$(BUILD).wasm

target/fbw-$(BUILD).wasm: ~/.cargo/bin/msfs_fix $(SOURCES) Makefile
	cargo build $(CARGO_FLAGS)
	msfs_fix target/wasm32-wasi/debug/fbw.wasm $@

~/.cargo/bin/msfs_fix:
	cargo install --git https://github.com/devsnek/msfs-rs --branch main msfs_fix
