-> Audiocoding in Rust <-
=======================
-> (with Pipes) <-
-> by major <-
-> (https://github.com/majorx234/rtaudio_example_rust) <-

-------------------------------------------------

```
         
       _~^~^~_
   \) /  o o  \ (/          Why Rust?
     '_   ¬   _'
     \ '-----' /
 ###################
 #-----------------#
 #-----------------#
 ###################         Why Pipes?
     #---------#
     #---------#
     #---------#
     #---------#
     ###########

```

-------------------------------------------------

-> Why Rust? <-
===============

- fast compiled language
- bindings for C-libraries
  - Jack, Alsa, Pipewire
- nice Vector implementation
  - slices (passing vectors by reference)
    - zero copy 

-------------------------------------------------

-> Why Pipes? <-
================

* longer story
                                               
▛▀▀▀▀▀▀▀▀▀▜          pipe          ▛▀▀▀▀▀▀▀▀▀▜ 
▌         ▐    ▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜    ▌         ▐ 
▌process1 ▐ -> ▌ stdout|stdin ▐ -> ▌process2 ▐ 
▌         ▐    ▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟    ▌         ▐ 
▙▄▄▄▄▄▄▄▄▄▟                        ▙▄▄▄▄▄▄▄▄▄▟ 

-------------------------------------------------

-> What is needed for a Synthesizer? <-
=======================================

- Oscilator (generates Waves)
- Filter, Effects
- ADSR (envelope)
- Play/ plot waves

▛▀▀▀▀▀▀▀▀▀▜         ▛▀▀▀▀▀▀▀▀▜        ▛▀▀▀▀▀▀▀▀▜        ▛▀▀▀▀▀▀▀▀▀▀▜
▌   OSC   ▐   ->    ▌ Filter ▐   ->   ▌  ADSR  ▐   ->   ▌ Playwave ▐
▙▄▄▄▄▄▄▄▄▄▟         ▙▄▄▄▄▄▄▄▄▟        ▙▄▄▄▄▄▄▄▄▟        ▙▄▄▄▄▄▄▄▄▄▄▟


                                           *
    |\  |\  |\         *   *   *          / \     *
    | \ | \ | \   ->  / \ / \ / \ /  ->  /   \   / \ /\___
    |  \|  \|  \         *   *   *            \ /   *
                                               *

-------------------------------------------------

-> # Let's build single modules and chain them
-> # with pipes!
                                               
▛▀▀▀▀▀▀▀▀▀▜          pipe          ▛▀▀▀▀▀▀▀▀▀▜ 
▌         ▐    ▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜    ▌         ▐ 
▌   OSC   ▐ -> ▌ stdout|stdin ▐ -> ▌   ADSR  ▐ 
▌         ▐    ▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟    ▌         ▐ 
▙▄▄▄▄▄▄▄▄▄▟                        ▙▄▄▄▄▄▄▄▄▄▟ 

## Write:

``` Rust
for sample in values_data {
    println!("{}", sample);
}
```

-------------------------------------------------

-> # Let's build single modules and chain them
-> # with pipes!
                                               
▛▀▀▀▀▀▀▀▀▀▜          pipe          ▛▀▀▀▀▀▀▀▀▀▜ 
▌         ▐    ▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜    ▌         ▐ 
▌   OSC   ▐ -> ▌ stdout|stdin ▐ -> ▌   ADSR  ▐ 
▌         ▐    ▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟    ▌         ▐ 
▙▄▄▄▄▄▄▄▄▄▟                        ▙▄▄▄▄▄▄▄▄▄▟ 

## Read:

```Rust
let mut values_data = std::io::stdin()
    .lock()
    .lines()
    .map(|x| x.expect("0.0").parse::<f32>().unwrap())
    .collect::<Vec<f32>>();
```

-------------------------------------------------

-> # DEMOTIME

-------------------------------------------------

-> # Future Ideas:
-> use Pipes bound to Filedescriptors:

* `./fm_wave  <(gen_wave 440 1.0)  <(gen_wave 10 1.0) |./play_wave`
  * avaible in *bash and *zsh
  * needs file handling <-> *stdin

-------------------------------------------------

-> # Future Ideas:
-> use named Pipes bound to Filedescriptors:

* `mkfifo pipe1`
* `mkfifo pipe2`
* `gen_wave 440 1.0 > pipe1`
* `gen_wave 10 1.0 > pipe2`
* `./fm_wave  <pipe1  <pipe2 |./play_wave`



