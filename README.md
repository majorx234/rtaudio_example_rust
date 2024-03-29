# rtaudio_example_rust
- rewrite of rtaudio_example (https://github.com/majorx234/rtaudio_example) in Rust
- learning Rust & rodio
- learning audio prgramming 

# Info
- Command Line Synthesizer
- pipe Data from one exe to another
- build with cargo:```cargo build```
- change path to ```targe/debug```
- use `./gen_wave` to generate a wave
  - `./gen_wave saw 660.0 2.0` -creates a saw wave with 660hz for 2 seconds
  - `./gen_wave invsaw 2093.0 0.25` -creates a inverted saw wave with 2093hz for 0.25 seconds  
  - `./gen_wave sine 440.0 0.5` -creates a sine wave with 440hz for 0.5 seconds
  - `./gen_wave tri 880.0 1.0` -creates a triangle wave with 880hz for 1 seconds
- adsr: pipe into `./adsr_modificator 0.05 0.2 0.5 0.25`
  - 4 paramaters for attack decay sustain and delay nneds to be set
- delay: simple delay (parameters delay time is 500ms, feedback is 0.33, both are hardcoded for now)
- overdrive: pipe into `./overdrive`
- overdrive_unsymetric: pipe into `./overdrive_unsymetric`
- WIP: filter
- sinks:
  - `play_wave`
  - `plot_wave` plot with help of plotly to html/svg (opens in browser)
  - `spectrogram 1024 512` generate a plot of frequency development over time (`window_size` = 1024, `step_size` = 512)
# examples:
- play a wav
```bash
./gen_wave saw 440 1.0 |./adsr_modificator 0.05 0.2 0.5 0.25|./play_wave
```
- plot a wav via GNU Plot
```bash
./gen_wave sin 440 0.1 |./adsr_modificator 0.05 0.2 0.5 0.25|gnuplot -p -e "set xrange[0:4800]; plot '-' "
```
- 2nd plot via GNU Plot
```bash
./gen_wave tri 440 0.2|./adsr_modificator 0.05 0.2 0.5 0.25|gnuplot -p -e "set xrange[0:9600]; plot '-' "
```
- play a fm wav
```bash
./gen_wave fm 50 6.0 |./adsr_modificator 0.05 0.2 0.5 0.25|./play_wave
```
- plot an overdrived wav:
```bash
./gen_wave sin 240 1.0|./overdrive_unsymetric|./plot_wave
```
- get spectrogram of frequency modulation:
```bash
./gen_wave fm 4400 1.0|.spectrogram 1024 512
```
# ToDo
- fix Filter
- play Sandstorm
- support named pipes/ piping via file descriptors
- "real time" - sound processing

# Links
- https://github.com/derekdreery/mixjack
  - taken idea for filter design
- https://github.com/Bujiraso/rickyhan.com-guitar-effects-in-rust
- https://github.com/0b01/rasta
  - taken idea for effect trait and code for overdrive, delay
- https://github.com/tsoding/haskell-music
  - thx to tsoding for inspiration of building a cool synth
- https://github.com/snd/stft
  - using the implementation of stft (updated to newer rustfft api)

# History
- 20230107 - Version 1.0.0 
  - data between nodes are now send without size information (pure samples)
