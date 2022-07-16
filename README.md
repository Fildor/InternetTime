# InternetTime

Just a fun little project to play around with different
programming languages.

Each sub-project is tasked with simply printing the current
Internet Time (aka "Swatch .Beats") to StdOut.

## Internet Time

Internet Time devides the day into 1000 "beats" and always
refers to Biel Mean Time, which is UTC+01:00 without daylight
savings.

Its value is computed by taking the utc day's seconds, add one hour,
modulo by 24 hours and devide by 86.4.

## Sub Projects

Currently, there are projects in

- C
- C#/NET Core 3.1
- C#/NET 6
- Python
- Rust
- Go
- Lua
- POSIX Shell Script
- Fish Shell Script

and I would be happy to add more to the list.

## Usage

What am I doing this for? - Well, long story short:
I wanted Internet Time in my i3blocks, so I wrote it.
Then I thought: "Why not do this in as many languages
as I possibly can?" and here we are.

Meanwhile, I also used those in PolyBar and Conky.

For what *you* may use it is up to you.

## License

This code is far from being rocket science and _anyone_ could have
come up with it (or better versions). So: Do as you please with it.

## Code of conduct
- No compelled speech
- No politics, no religion
- Don't be an A**
