pdflatex-args = 'pdflatex -interaction=nonstopmode -shell-escape'

.PHONY: all
all: main.pdf

.PHONY: clean
clean:
	latexmk -CA
	rm -fr _minted-main *.{bbl,codesnippet{,s},gls.*,run.xml}

main.pdf: main.tex glossary.bib references.bib
	latexmk -pdf -pdflatex=$(pdflatex-args) -use-make $<

.PHONY: open
open: main.pdf
	nohup xdg-open $< >/dev/null 2>&1 &