debug ?=
release = $(if $(debug),,--release)

.PHONY: all
all: build

.PHONY: clean clean-cargo clean-doc
clean: clean-cargo clean-undergraduate-thesis
clean-cargo: ; cargo clean
clean-undergraduate-thesis: ; $(MAKE) -C doc/undergraduate-thesis clean

.PHONY: build
build: ; cargo build $(release)

.PHONY: doc doc-cargo doc-defense-seminar doc-undergraduate-thesis
doc: doc-cargo doc-defense-seminar doc-undergraduate-thesis
doc-cargo: ; cargo doc
doc-defense-seminar: ; $(MAKE) -C doc/undergraduate-thesis defense-seminar
doc-undergraduate-thesis: ; $(MAKE) -C doc/undergraduate-thesis

.PHONY: open-undergraduate-thesis
open-undergraduate-thesis: ; $(MAKE) -C doc/undergraduate-thesis open

.PHONY: present-defense-seminar
present-defense-seminar:
	$(MAKE) -C doc/undergraduate-thesis present-defense-seminar