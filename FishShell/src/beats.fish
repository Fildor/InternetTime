#!/usr/bin/fish

set dayseconds (math "24*3600")
set utcSeconds (date -u +%s)
set beats (math "$utcSeconds+3600")
set beats (math "$beats%$dayseconds")
set beats (math -s 1 "$beats/86.4")
echo $beats
set -e beats
set -e dayseconds
set -e utcSeconds
