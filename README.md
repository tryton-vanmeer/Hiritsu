# Hiritsu (比率 Ratio)

Python tool for getting aspect ratio of files.


```bash
$ hiritsu --help
Usage: hiritsu [-hv] FILE

-h --help       show this
-v --verbose    show more info

$ hiritsu file.png
16/9

$ hiritsu -v file.png
Width: 1920
Height: 1080
Ratio: 16/9
```

### Dependencies

**Debian/Ubuntu**

`$ apt install python3-docopt, python3-pil`

**Fedora**

`$ dnf install python3-docopt python3-pillow`

**Arch Linux**

`$ pacman -S python-docopt python-pillow`
