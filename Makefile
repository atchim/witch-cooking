all: debug

clean:
	cargo clean
	$(MAKE) -C doc/undergraduate-thesis clean

debug:
	cargo build

doc:
	cargo doc

doc-all: doc doc-undergraduate-thesis

doc-undergraduate-thesis:
	$(MAKE) -C doc/undergraduate-thesis

open-undergraduate-thesis:
	$(MAKE) -C doc/undergraduate-thesis open

release:
	cargo build --release

.PHONY: all clean debug doc doc-all doc-undergraduate-thesis
.PHONY: open-undergraduate-thesis release