.PHONY: build install

dont-replace := false

build: target/wheels/cubism*

install: build
	[ "$(dont-replace)" == "false" ] && yes | pip uninstall cubism
	pip install ./target/wheels/cubism*

target/wheels/cubism*: src/*.rs
	CUBISM_CORE=$$(realpath ./cubism-sdk/) maturin build --manylinux=off
