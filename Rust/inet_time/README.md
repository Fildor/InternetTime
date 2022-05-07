# InetTime in Rust

Gibt die aktuelle Uhrzeit in Swatch-Beats aus.
D.h. BMT in 1000stel Anteile von 24h ab 00:00

## Umrechnung

Die Umrechnung erfolgt nach der Formel

$.beats = ((secondsOfDayInUTC + 01:00) MOD 24h) / 86.4$

## Installation

### i3-blocks

1. Projekt im Release Modus bauen: `cargo build --release`
2. Im VZ ./target/release die Exe "inet_time" umbenennen in "beats".
   `cp ./target/release/inet_time ./target/release/beats`
3. "beats" kopieren: `cp ./target/realease/beats ~/.config/i3blocks`
4. Dort Block hinzufügen:

```
#internet time
[beats]
label=<your label here>
command=~/.config/i3blocks/beats
interval=5
```

### Polybar

1. Projekt im Release Modus bauen: `cargo build --release`
2. Executable ins Polybar scripts VZ kopieren.
3. Dort entsprechendes Modul definieren.
