# rtaudio_example_rust
- rewrite of rtaudio_example (https://github.com/majorx234/rtaudio_example) in Rust
- learning Rust & rodio
- learning audio prgramming 

## Info
- Command Line Synthesizer
- pipe Data from one exe to another
- use `./gen_wave` to generate a wave
  - `./gen_wave saw 660.0 2.0` -creates a saw wave with 660hz for 2 seconds
  - `./gen_wave invsaw 2093.0 0.25` -creates a inverted saw wave with 2093hz for 0.25 seconds  
  - `./gen_wave sine 440.0 0.5` -creates a sine wave with 440hz for 0.5 seconds
  - `./gen_wave tri 880.0 1.0` -creates a triangle wave with 880hz for 1 seconds
- adsr: pipe into `./adsr_modificator`
- sinks:
  - `play_wave`

## examples:
- play a wav
```bash
./gen_wave saw 440 1.0 |./adsr_modificator|./play_wave
```
- plot a wav via GNU Plot
```bash
./gen_wave sin 440 0.1 |./adsr_modificator|gnuplot -p -e "set xrange[1:4800]; plot '-' "
```
- 2nd plot via GNU Plot
```bash
./gen_wave tri 440 0.2|./adsr_modificator|gnuplot -p -e "set xrange[1:9600]; plot '-' "
```
- play a fm wav
```bash
./gen_wave fm 50 6.0 |./adsr_modificator|./play_wave
```

## Links
- https://github.com/derekdreery/mixjack
  - taken idea for filter design
- https://github.com/Bujiraso/rickyhan.com-guitar-effects-in-rust
- https://github.com/0b01/rasta
  - taken idea for effect trait and code for overdrive
