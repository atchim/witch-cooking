main-files = $(wildcard main.*)
disposable-main-files = $(filter-out main.pdf main.tex,$(main-files))
pdflatex-args = 'pdflatex -interaction=nonstopmode -shell-escape'

.PHONY: all
all: main.pdf

.PHONY: clean
clean:
	latexmk -CA
	rm -fr _minted-main $(disposable-main-files)

main.pdf: main.tex glossary.bib references.bib
	latexmk -pdf -pdflatex=$(pdflatex-args) -use-make $<

.PHONY: open
open: main.pdf ; nohup xdg-open $< >/dev/null 2>&1 &