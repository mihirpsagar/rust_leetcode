#!/bin/bash
sed '10q;d' file.txt
# i=1
# while read LINE; do
#     if [ $i -eq 10 ]; then
#         echo $LINE
#         break
#     fi
#     i=$((i + 1))
# done <file.txt
