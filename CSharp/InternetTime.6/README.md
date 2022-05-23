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
1. Copy executable to ~/.config/polybar/scripts/
2. Add a custom module (change to your liking)

```
[module/beats]
type = custom/script
exec = ~/.config/polybar/scripts/<executable name>
interval = 5
label = .Beats: %output%
format-foreground = ${colors.foreground}
format-background = ${colors.background}
format-prefix = ""
format-prefix-foreground = #FFBB00
format-underline = #FFBB00
```

3. In the section about your WM add "beats" to the line
```
modules-right = pavolume memory2 cpu2 date beats
```
or `modules-center` or `modules-left` - wherever you want it.