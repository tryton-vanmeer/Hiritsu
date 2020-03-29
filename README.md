# Hiritsu (比率 Ratio)

```bash
$ hiritsu --help
hiritsu 1.0.0
Tryton Van Meer <trytonvanmeer@protonmail.com>
Gets the aspect ratio and resolution of images.

USAGE:
    hiritsu [FLAGS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -r, --rename     Rename the image
    -V, --version    Prints version information

ARGS:
    <FILE>    Sets the image to use

$ hiritsu wallpaper.jpg
Width:  1920
Height: 1080
Ratio:  16:9

$ hiritsu --rename wallpaper.jpg
wallpaper (1920x1080) [16:9].jpg
```
