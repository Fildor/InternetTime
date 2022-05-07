# Internet Time in C# / .Net 6

This program output the current Internet Time to STD_OUT.

## What _is_ Internet Time?

Also known as "Swatch .beats", Internet Time is "Biel Mean Time" (UTC +1)
in seconds past midnight devided by 86.4. This devides a day into 1000 ".beats".

This program will output this value preceeded by a "@" with a fraction of 10th.
Example:
"6 AM" in Berlin ("Wintertime" = without Daylight Savings) = "@250.0"
"5 AM" UTC = "@250.0"

## Installation

This program is dependent on the dotnet runtime ... TBD

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
