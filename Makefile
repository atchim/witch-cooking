debug ?=
release = $(if $(debug),,--release)

.PHONY: all
all: build

.PHONY: clean clean-cargo clean-doc
clean: clean-cargo clean-doc
clean-cargo: ; cargo clean
clean-doc: ; $(MAKE) -C doc/undergraduate-thesis clean

.PHONY: build
build: ; cargo build $(release)

.PHONY: doc doc-cargo doc-undergraduate-thesis
doc: doc-cargo doc-undergraduate-thesis
doc-cargo: ; cargo doc
doc-undergraduate-thesis: ; $(MAKE) -C doc/undergraduate-thesis

.PHONY: open-undergraduate-thesis
open-undergraduate-thesis: ; $(MAKE) -C doc/undergraduate-thesis open