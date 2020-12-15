#!/bin/sh

echo 'Started conversion.'
echo ''

pandoc -s --columns=1 --template=latex-fixed.template -o 'MS_Harmonogram_4D_Najman_Jan.pdf' harmonogram.md

echo ''
echo 'Finished.'
