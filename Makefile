all: debug

clean:
	cargo clean
	$(MAKE) -C doc/undergraduate-thesis clean

debug:
	cargo build

doc:
	cargo doc

doc-all: doc undergraduate-thesis

open: undergraduate-thesis
	open doc/undergraduate-thesis/main.pdf

release:
	cargo build --release

undergraduate-thesis:
	$(MAKE) -C doc/undergraduate-thesis

.PHONY: all clean debug doc doc-all release undergraduate-thesis