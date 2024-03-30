# Instructions

**Content**

- [Internet Time](#internet-time-aka-swatch-beats)
- [Calculation](#calculation-from-utc-timestamp)
- [Build from source](#build-from-source-code)
  - [ C](#c)
  - [ C#/.Net](#cnet)
  - [󰈺 Fish](#fish)
  - [󰟓 Golang](#go)
  - [󰢱 Lua](#lua)
  - [ POSIX Shell](#posix-shell)
  - [󰌠 Python](#python)
  - [󱘗 Rust](#rust)
  - [ Zig](#zig)
- [Installation](#installation)
  - [Conky](#conky)
  - [neofetch](#neofetch)
  - [ i3blocks](#i3-blocks)
  - [Polybar](#polybar)
  - [Notofication](#as-notification-dunst)

## Internet Time a.k.a. Swatch Beats

Each program in this collection has one single simple purpose:
Output the current Internet Time. That is a string in the format
`@000.0`. The `@` denotes the Internet Time, the following number is
the current time in 1000th part of a day rounded to 1 decimal place.
The reference time is BMT (Biel (, Switzerland) Mean Time) which is UTC +1 
_without_ daylight-savings.

## Calculation from UTC Timestamp

$$
.beats = \frac{(secondsOfDayInUTC + 01:00) MOD 24h}{86.4}
$$

## Build from source code

The following instructions presuppose you have all neccessary tools for 
building a simple program in the respective languages installed.

### C

You can make use of the makefile `make all` which puts the executable into the
`./bin` subfolder.

### C#/.Net

```
dotnet build -c Release
```

### Fish

Nothing to build. Just copy to or link to the script from `$HOME/.config/fish/functions` 

### GO

Either build directly with 

```
go build -o ./bin/beats ./src/main.go 
```
or use the included Taskfile (see https://taskfile.dev):

```
task build
```
which will do the same.

### Lua

Nothing to build.
Execute via `lua beats.lua` or use the shell script.

### POSIX-Shell

Nothing to build. 
Just copy to or link to the script from somewhere that's on PATH.

### Python

Nothing to build.
Execute via `python main.py` or use the shell script.

### Rust

To build in release mode:

```
$ cargo build --release
```
The artifact can be copied from or linked to in `./target/release/inet_time`

### Zig

TODO

## Installation

Easiest (imho) would be to just add a symbolic link to the build artefact from
the language of your choice. The link source should be in your PATH.

For example: you could link
$HOME/.local/bin/beats -> $HOME/GitHub/InternetTime/LANG/bin/Release/beats

### Conky

TODO

### neofetch

TODO

### i3-blocks

Add this block to your config:

```
#internet time
[beats]
label=<your label here>
command=beats
interval=5
```
If you do not want to have it in your PATH, you can make a symbolic link to 
the executable from `.config/i3blocks/beats` and use that as value in `command`.

### Polybar

Presupposing you have `beats` on your PATH _or_ copy to _or_ link from 
`~/.config/polybar/scripts/beats`:

Add a custom module like this (change to your liking, of course)

```
[module/beats]
type = custom/script
exec = ~/.config/polybar/scripts/beats
interval = 5 
label = %output%
format-foreground = ${colors.foreground}
format-background = ${colors.background}
```

In the section about your Window Manager, add "beats" to the line ...
```
modules-right = pavolume memory2 cpu2 date beats
```
or `modules-center` or `modules-left` - wherever you want it.

### As notification / Dunst

Example, presupposing you have `beats` in your PATH:

Either
```
notify-send "Title" $(beats)
```
or
```
dunstify "Title" $(beats)
```

For further customization and/or mapping to a hot-key refer to the respective
manuals of your WM, HotKey-Manager and Dunst (or other Notification-Client).
