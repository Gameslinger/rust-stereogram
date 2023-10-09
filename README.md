# Rust Stereogram Generator
!(An autostereogram of a shark)[stereogram.png]
I think sterograms are underrated. It is amazing how our eyes can infer depth from a picture of static. I decided to write this random-dot autostereogram generator for fun and to learn rust. I attached some depth maps I found online. I hope this weekend project can spark an interest in stereograms and rust!

## Explanation
The algorithm for this program is based off of the code found [here](https://flothesof.github.io/making-stereograms-Python.html). Basically, a pattern of width ```[pattern_width]``` is generated and repeats over the width of the image. Depending on the brightness of the depth map, the pixel gets moved to the left in that repeating pattern(controlled by ```[shift_amplitude]```). Our eyes can recognize the shifted pixels from the background noise and provide the illusion of depth. For a much better explanation, check out the [wikipedia page](https://en.wikipedia.org/wiki/Autostereogram).

```bash
# Building
cargo b -r
mv target/release/autostereogram .
# Usage:
./autostereogram [out width] [out height] [pattern_width] [shift_amplitude] [depthmap] [output name]

# Examples
./autostereogram 600 400 40 0.6 shark.png shark-stereogram.png
./autostereogram 600 600 40 0.6 elephant.png elephant-stereogram.png
```
