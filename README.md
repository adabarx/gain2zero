# Gain2zero

## Concept

Inspired by [Baphometrix's clip to zero mixing strategy](https://www.youtube.com/watch?v=5UT42-ur080&list=PLxik-POfUXY6i_fP0f4qXNwdMxh3PXxJx) Gain2Zero is a simple plugin that decreases the gain on an audio signal until the loudest samples are at or below 0dbfs. This is a simple plugin i made to assist with gain staging. It displays the current gain attenuation in decibels on the GUI and then I take that information and adjust my audio processing from there.

There are no parameters, just a reset button in the GUI that sets the current attenuation back to 0db.

## Building

After installing [Rust](https://rustup.rs/), you can compile Gain2zero as follows:

```shell
cargo xtask bundle Gain2Zero --release
```
