# InternetTime in Python

Prints the current Internet Time to STD_OUT

## What _is_ the current Intenret Time?

Also known as "Swatch .beats", the Internet Time is:
Biel Mean Time (UTC+1) in seconds past midnight devided by 86.4.

This devides a day in 1000 ".beats" that are displayed as "@XXX"
or if you want it more precisely and as is output by this program:
"@XXX.X" (with a fraction)

## Build

This program is written in Python and can be directly be started with
`python main.py` or the convenience start script `beats.sh`.

## Usage in Bars

### i3-blocks

1. Copy executable to ~/.config/i3blocks
2. Add to your `config` file:

```
#internet time
[beats]
label=<your label here>
command=~/.config/i3blocks/beats
interval=5
```

### Polybar

TBD
