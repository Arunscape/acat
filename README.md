# Arun's cat

Usage: clone the repo and run 
```
echo hello | cargo run
```

And if you want to install it for whatever reason:

```
cargo install acat
```

Or, if you're an Arch user, use the PKGBUILD

```
makepkg -si
```

```
echo hello | acat
acat file1.txt file2.txt
```

```
USAGE:
    acat [FLAGS] [OPTIONS] [FILE]...

FLAGS:
    -A, --show-all            equivalent to -vET
    -b, --number-nonblank     number nonempty output lines, overrides -n
    -e                        equivalent to -vE
    -E, --show-ends           display $ at end of each line
    -n, --number              number all output lines
    -s, --squeeze-blank       suppress repeated empty output lines
    -t                        equivalent to -vT
    -T, --show-tabs           display TAB characters as ^I
    -u                        (ignored)
    -v, --show-nonprinting    use ^ and M- notation, except for LFD and TAB
    -h, --help                Prints help information
    -V, --version             Prints version information

OPTIONS:
        --hue <hue>                   [possible values: mono, red, orange, yellow, green, blue, purple, pink]
        --luminosity <luminosity>     [possible values: bright, light, dark, random]

ARGS:
    <FILE>...    File(s) to print / concatenate. Use '-' for standard input.
```
