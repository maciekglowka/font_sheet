### A simple tool converting TTF font files into bitmap sprite sheets.

The resulting atlas is aligned to ASCII indices. Capital 'A' will be placed as sprite no. 65 etc.


```
Usage: font_sheet [OPTIONS] <PATH> <SIZE>

Arguments:
  <PATH>  Input font path
  <SIZE>  Em size in px

Options:
  -c <COLUMNS>                         Sprite sheet columns [default: 16]
  -o <OUTPUT>                          Output path [default: <input_path>.png]
      --color <COLOR> <COLOR> <COLOR>  Color [default: 255 255 255]
      --v-gap <V_GAP>                  Vertical gap [default: 0]
      --h-gap <H_GAP>                  Horizontal gap [default: 0]
  -h, --help                           Print help
  -V, --version                        Print version
```
