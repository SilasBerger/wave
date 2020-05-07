# Wave
Wave is a simple text-based synthesizer written in Rust. Note that this is mostly a toy project and an exercise, rather than a productive application.

## Repo Structure
- `wave`: Core code base, contains the Rust project.
- `wave-vis`: Utility tool for visualizing a wave form in `.pcm` format. Mostly irrelevant at this point.

## Usage
All instructions in this section refer to the `./wave` directory, where the Rust project is located. 

First, build Wave by running `cargo build --release`. Then, use `target/release/wave` (or `target/release/wave.exe` on Windows) for all further steps, wherever the command `wave` is used.

To get started, it's easiest to use one of the pre-made input files. To generate an output file `./the_licc.wav` from an input file `./input_files/the_licc.wss`, run

```
wave input_files/the_licc.wss the_licc.wav
```

Note that the second argument is optional. If it's not supplied, the output file will be `./output.wav` by default.

### Writing Custom Input Files
To write custom input files, it's easiest to copy one of the pre-made input files as a starting point. Note that the `#header` and `#data` section both need to be present and their order must not be switched. Also note that lines starting with `//` are considered comments and will be ignored by the parser. Partial-line comments are not supported.

#### Header
The header section consists of a list of `key=value` fields. The following fields must be present in the header section:
- `sample_rate`: Number of samples per second, `u16`.
- `freq_a4`: Frequency of the reference pitch A4, `f64`.
- `subdivision`: Number of note subdivisions per bar (e.g. `subdivision=8` -> 8 8th notes per bar), `u8`.
- `bpm`: Tempo of the track, in beats per minute, `u16`.
- `volume`: Volume of the resulting track, in [0, 1], `f64`.

#### Data
The data section consists of an unlimited number of lines, where each line represents a "fragment". A fragment is either a single note, chord (group of notes), or rest, to be played for a given number of subdivisions (note value). Each line has the following format:

`<value> <note> [note] ... [note]`

So, each line needs to start with a note value (e.g. 2, for a quarter note, if `subdivision=8` in the header), followed by an arbitrary number of notes. Notes have the following format:

`<note_name><octave>[detune]`

Acceptable note names consist of a single upper-case letter, followed by a `#` for sharps, or a `b` for flats. Notes such as `Cb` (=B) or `E#` (=F) are allowed as well. The `octave` value is a number referring to the octave in which to play a given note. E.g. `A4` represents a note `A` in the 4th octave. Finally, a `detune` value can be used to detune a note by a given number of cents, upward or downward. For instance `A4+20` is a note `A4`, detuned by +20 cents, and `C5-12` is a note `C5`, detuned by -12 cents. Note that there is no white-space between note names, octaves and detune specifiers. Additional white-space between notes, or between the value and the first note is ignored.

To specify a rest, instead of a note or chord, use `-` in place of the first note specifier. For instance, a line which represents a rest with a duration of 4 subdivisions would be denoted as

`4 -`

For examples of how to use rests, check `input_files/the_licc_with_rests.wss`. For an example with detuned notes, check `input_files/the_licc_detuned.wss`.

## Current Issues
- Input file parsing theoretically allows for rests + pitches in the same fragment / chord.
- Some functions don't return results yet.
- Audio clicks at fragment boundaries.
- Input file format doesn't currently support changing volume per fragment.
- Numeric input values (octave, detune, fragment value) are never bounds-checked. Need to make sure that they a) don't exceed the data type bounds and b) are always semantically valid (e.g. maybe prevent detune of more than +/-100 cents).
- Note value is currently not well define: semantically, it should be okay to have a value of e.g. 16, even if the sub-division is only 8 (i.e. 2 bars). However, need to make sure that this works properly, and that the data type is large enough to support longer note values.

## Educational Resources Used
- [Write a Wave File in Rust](https://www.youtube.com/watch?v=odeWLp96fdo)
- [Audio Programming for Beginners Tutorial 02 - The Sine Wave](https://www.youtube.com/watch?v=Yk9CRHntoiI)
