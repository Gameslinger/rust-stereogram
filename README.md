# Rust Stereogram Generator
![An autostereogram of a shark](stereogram.png)

I think sterograms are underrated. It is amazing how our eyes can infer depth from a picture of static. I decided to write this random-dot autostereogram generator for fun and to learn rust. I attached some depth maps I found online. I hope this weekend project can spark an interest in stereograms and rust!

## Explanation

The algorithm for this program is based off of the code found [here](https://flothesof.github.io/making-stereograms-Python.html). Basically, a pattern of width ```[pattern_width]``` is generated and repeats over the width of the image. Depending on the brightness of the depth map, the pixel gets moved to the left in that repeating pattern(controlled by ```[shift_amplitude]```). Our eyes can recognize the shifted pixels from the background noise and provide the illusion of depth. For a much better explanation, check out the [wikipedia page](https://en.wikipedia.org/wiki/Autostereogram).

## Building and Running

```bash
# Building
cargo b -r
mv target/release/autostereogram .

# Examples
./autostereogram --depth-file=shark.png --out-image=shark-stereogram.png --width=600 --height=400 --pattern-size=40 --deepness=0.6 
./autostereogram -d elephant.png
```
### Usage

Only parameter --depth-file (-d) is required. Other options uses hardcoded values or calculated from input depth image.
```
Usage: autostereogram.exe [OPTIONS] --depth-file <IMAGE_FILENAME>

Options:
  -d, --depth-file <IMAGE_FILENAME>  Image file name that represent depth distribution
  -o, --out-image <OUTPUT_FILENAME>  Stereogram file name [default: --depth-file=bulgy.jpg => bulgy-stereogram.jpg]
  -p, --pattern-size <PATTERN_SIZE>  Size of repeating dots pattern square (in pixels) [default: 40]
  -D, --deepness <DEEPNESS>          Pattern distortion to represent depth. [0.1 - 0.9] [default: 0.6]
  -W, --width <WIDTH>                Width of outer image (in pixels)
  -H, --height <HEIGHT>              Height of outer image (in pixels)
  -h, --help                         Print help
  -V, --version                      Print version
```