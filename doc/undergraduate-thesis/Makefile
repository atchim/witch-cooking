.PHONY: all
all: main.pdf

.PHONY: clean
clean:
	latexmk -CA
	rm -fr main.bbl main.gls.* main.run.xml

main.pdf: main.tex glossary.bib references.bib
	latexmk -pdf -pdflatex="pdflatex -interaction=nonstopmode" -use-make $<

.PHONY: open
open: main.pdf
	nohup xdg-open $< >/dev/null 2>&1 &