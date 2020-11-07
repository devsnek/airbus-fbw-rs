SOURCES := $(wildcard ./fbw/**/*) $(wildcard ./model/**/*) $(wildcard ./airbus-fly-by-wire-wasm/src/model/**/*)
BUILD ?= debug
CARGO_FLAGS = --target wasm32-wasi
WASM_OPT = : # null command
export RUSTFLAGS = -Clink-arg=--export-table

ifeq ($(BUILD),release)
	CARGO_FLAGS += --release
	ifneq (,$(shell command -v wasm-opt 2> /dev/null))
		WASM_OPT = wasm-opt
	endif
endif

ifeq (,$(shell command -v msfs_fix 2> /dev/null))
	_ := $(shell cargo install --git https://github.com/devsnek/msfs-rs --branch main msfs_fix)
endif

all: target/fbw-$(BUILD).wasm

target/fbw-$(BUILD).wasm: $(SOURCES) Makefile
	cargo build $(CARGO_FLAGS)
	msfs_fix target/wasm32-wasi/$(BUILD)/fbw.wasm $@
	$(WASM_OPT) -O4 $@ -o $@
