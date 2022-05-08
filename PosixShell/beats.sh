#!/bin/sh

dayseconds=$((24*3600))
utcseconds=$(date -u +%s)
result=$((utcseconds + 3600))
result=$((result%dayseconds))
result=$(echo "$result/86.4" | bc -l)
printf '%.1f' "$result" 2>/dev/null
