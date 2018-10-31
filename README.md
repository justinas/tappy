# Tappy

Tappy is a console (text user interface) based tap BPM (beats per minute) meter.

[![asciicast](https://asciinema.org/a/4VIzywVeQ0vlj9RGTswSxxfRh.png)](https://asciinema.org/a/4VIzywVeQ0vlj9RGTswSxxfRh)

## Installation

If you have Cargo installed, installation is as simple as running `cargo install tappy`.
Tappy will work on any platform supported by [Termion](https://github.com/redox-os/termion).

Standalone binary releases will be coming soon.

## Usage

Tappy has no options. Just launch `tappy` from the terminal
and tap your spacebar key to the rhythm.
Press backspace to reset the calculations or Q to quit.

Tappy will show both the momentary and average BPM
as well as the average BPM rounded to the nearest integer.

## Performance

On my i7-6700k holding the spacebar down maxes out the BPM at about 2000
with GNOME Terminal using about 15% of a single CPU core,
meanwhile Tappy uses about 3%.

This means you can use Tappy to analyze even very, *very* fast songs.

## Acknowledgements

The set of functionality has been ripped of from the
[tap BPM meter by All8](https://www.all8.com/tools/bpm.htm).
