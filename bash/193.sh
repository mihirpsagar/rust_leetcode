#! /usr/bin/bash
grep -e "^([0-9]\{3\}) [0-9]\{3\}\-[0-9]\{4\}$" -e "^[0-9]\{3\}\-[0-9]\{3\}\-[0-9]\{4\}$" file.txt
# exec <file.txt
# while read line; do
#     [[ $line = \([0-9][0-9][0-9]\)\ [0-9][0-9][0-9]-[0-9][0-9][0-9][0-9] || $line = [0-9][0-9][0-9]-[0-9][0-9][0-9]-[0-9][0-9][0-9][0-9] ]] && echo "$line"
# done
