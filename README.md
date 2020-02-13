# Arun's cat

Note: it doesn't actually open or concatenate files lol,
all it does is you pipe something to its stdin and it outputs
random colours


Usage: clone the repo and run 
```
echo hello | cargo run
```

And if you want to install it for whatever reason:

```
cargo install --path .
```

```
echo hello | acat
acat file1.txt file2.txt
```
