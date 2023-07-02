# cat-rs

My rewrite of the cat GNU core utility in rust

version: 0.1.0
author: [Lu Baumann](https://blog.surferlul.me/)

## usage

refer to `cat-rs -h`:

```
Concatenate file(s) to standard output

Usage: cat-rs [OPTIONS] [FILES]...

Arguments:
  [FILES]...  With no file, or when file is -, read standard input

Options:
  -A, --show-all          equivalent to -vET
  -b, --number-nonblank   number nonempty output lines, overrides -n
  -e                      equivalent to -vE
  -E, --show-ends         display $ at end of each line
  -n, --number            number all output lines
  -s, --squeeze-blanks    suppress repeated empty output lines
  -t                      equivalent to -vT
  -T, --show-tabs         display TAB character as ^I
  -u                      (ignored)
  -v, --show-nonprinting  use ^ and M- notation, except for LFD and TAB
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version
```

## testing

test script requires python >= 3.10

`gnu_cat_tests/test.py -e /bin/cat -b <path/to/cat-rs>`

| parameter        | value                                  |
| ---------------- | -------------------------------------- |
| `-c`, `--cat`    | path to cat executable to test against |
| `-b`, `--binary` | path to cat-rs binary to test          |
