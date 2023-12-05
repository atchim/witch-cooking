demo-name ?=
main-files = $(wildcard main.*)
disposable-main-files = $(filter-out main.pdf main.tex,$(main-files))
pdflatex-args = 'pdflatex -interaction=nonstopmode -shell-escape'

.PHONY: all
all: main.pdf

.PHONY: clean
clean:
	latexmk -CA
	rm -fr $(disposable-main-files)

.PHONY: demo
demo: ; ./demo/demo ./demo/"$(demo-name)".sh

main.pdf: main.tex ../references.bib
	latexmk -pdf -pdflatex=$(pdflatex-args) -use-make $<

.PHONY: present
present: main.pdf ; nohup zathura --mode presentation $< >/dev/null 2>&1 &