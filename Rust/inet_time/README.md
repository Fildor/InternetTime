# InetTime in Rust

Gibt die aktuelle Uhrzeit in Swatch-Beats aus.
D.h. BMT (Biel Mean Time = UTC +01:00 ohne Sommerzeit) in 1000stel Anteilen von 
24h ab 00:00 Uhr.

## Umrechnung

Die Umrechnung erfolgt nach der Formel

$.beats = ((secondsOfDayInUTC + 01:00) MOD 24h) / 86.4$

## Installation

### i3-blocks

1. Projekt im Release Modus bauen: `cargo build --release`
2. Aus dem Verzeichnis ./target/release die ausführbare Datei "inet_time" kopieren
   und umbenennen: `cp ./target/realease/inet_time ~/.config/i3blocks/beats` oder
   einen Link erstellen: `ln -s ./target/release/inet_time $HOME/.config/i3blocks/beats`
3. Dort Block hinzufügen:

```
#internet time
[beats]
label=<your label here>
command=~/.config/i3blocks/beats
interval=5
```

### Polybar

1. Build project in Release mode: `cargo build --release`
2. Copy executable to ~/.config/polybar/scripts/
   `cp ./target/release/inet_time ~/.config/polybar/scripts/beats`
3. Add a custom module (change to your liking)

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

4. In the section about your WM add "beats" (or whatever you called it) to the 
   line ...
```
modules-right = pavolume memory2 cpu2 date beats
```
or `modules-center` or `modules-left` - wherever you want it.

### As notification
Given you copy or link the executable to your .local/bin:
`notify-send "@beats" $(~/.local/bin/beats)`
