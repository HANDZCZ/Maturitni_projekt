#!/bin/sh

echo 'Started conversion.'
echo ''

pandoc -s --filter pandoc-crossref --template=latex-fixed.template --columns=1 -o '2021_4D_MP_NajmanJan_Piskvorky.pdf' dokumentace.md

echo ''
echo 'Finished.'
