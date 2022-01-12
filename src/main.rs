/*
 * This file is part of the rtaudio_example_rust distribution (https://github.com/majorx234/rtaudio_example_rust ).
 * Copyright (c) 2022 Majorx234
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::env::args;
mod saw_wave;
mod wave;

use saw_wave::SawWave;
use wave::Wave;
fn main() {
    let mut argit = args();
    let freq = argit.nth(1).clone();
    let duration = argit.next().clone();

    let freq = if let Some(freq) = freq {
        if let Ok(freq) = str::parse::<u32>(&freq) {
            freq
        } else {
            panic!("frequency isn't given as unsigned int value");
        }
    } else {
        panic!("No frequency argument given");
    };

    let duration = if let Some(duration) = duration {
        if let Ok(duration) = str::parse::<f32>(&duration) {
            duration
        } else {
            panic!("duration isn't given as float value");
        }
    } else {
        panic!("No duration argument given");
    };

    let fsample_rate: f32 = 48000.0;
    let fnum_samples = fsample_rate * duration;
    let num_samples = fnum_samples as usize;
    let mywave = SawWave::new(freq, num_samples);
    mywave.print();
    // debuging println!("{:?}", mywave);
}
