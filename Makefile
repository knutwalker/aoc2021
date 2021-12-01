# https://tech.davis-hansson.com/p/make/
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

APP := aoc2021

CARGOFLAGS ?=

# generate release build
all: build
build: target/release/$(APP)

# clean build output
clean: .cargoinstalled
> cargo clean

.PHONY: all build clean

### build targets

target/release/$(APP): .cargoinstalled Cargo.toml Cargo.lock $(shell find src -type f)
> RUSTFLAGS="-C link-arg=-s -C opt-level=3 -C target-cpu=native --emit=asm" cargo build $(CARGOFLAGS) --bin $(APP) --release

.cargoinstalled:
> @if ! command -v cargo 2> /dev/null
> @then
>   @echo "Cargo is not installed. Please visit 'https://rustup.rs/' and follow their instructions, or try to run 'curl --proto \"=https\" --tlsv1.2 -sSf https://sh.rustup.rs | sh'"
>   @exit 1
> @fi
> touch .cargoinstalled
