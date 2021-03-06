#!/bin/sh

echo 'Started conversion.'
echo ''

#pandoc -s --filter pandoc-crossref --template=latex-fixed.template --columns=1 -o '2021_4D_MP_NajmanJan_Piskvorky.pdf' dokumentace.md
#pandoc -s --filter pandoc-crossref --template=latex-fixed.template -o '2021_4D_MP_NajmanJan_Piskvorky.pdf' dokumentace.md
#pandoc -s --filter pandoc-crossref --template=latex-fixed.template --pdf-engine lualatex -o '2021_4D_MP_NajmanJan_Piskvorky.pdf' dokumentace.md
pandoc -s --filter pandoc-crossref --template template.latex --pdf-engine lualatex --bibliography bib.yaml --citeproc --csl iso690-numeric-brackets-cs.csl -o '2021_4D_MP_NajmanJan_Piskvorky.pdf' dokumentace.md

echo ''
echo 'Finished.'
