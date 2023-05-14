all: debug

clean:
	cargo clean
	$(MAKE) -C doc/undergraduate-thesis clean

debug:
	cargo build

doc:
	cargo doc

doc-all: doc doc-thesis

doc-thesis:
	$(MAKE) -C doc/undergraduate-thesis

open-thesis:
	$(MAKE) -C doc/undergraduate-thesis open

release:
	cargo build --release

.PHONY: all clean debug doc doc-all doc-thesis open-thesis release