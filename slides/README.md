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
  - Jack, Alsa
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

▛▀▀▀▀▀▜    ▛▀▀▀▀▀▀▀▀▜    ▛▀▀▀▀▀▀▜    ▛▀▀▀▀▀▀▀▀▀▀▜
▌ OSC ▐ -> ▌ Filter ▐ -> ▌ ADSR ▐ -> ▌ Playwave ▐
▙▄▄▄▄▄▟    ▙▄▄▄▄▄▄▄▄▟    ▙▄▄▄▄▄▄▟    ▙▄▄▄▄▄▄▄▄▄▄▟


                                       *
    |\  |\  |\         *   *          / \
    | \ | \ | \   ->  / \ / \ /  ->  /   \  /\___
    |  \|  \|  \         *   *            \/


-------------------------------------------------

-> # Let's build single modules and chain them
-> # with pipes!
