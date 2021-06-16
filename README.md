# Reverse-engineering Apple's JP, JP-EN and JP-CN Dictionaries

## Why?

I want to use Anki to learn Japanese vocabulary. However, manually editing the cards is a pain. I want a script that takes a word as the input and generate a card with detailed definitions for me and add the card to the deck (can be achieved via [anki-connect](https://github.com/FooSoft/anki-connect)). Parsing definition webpages from e.g. [goo dictionary](dictionary.goo.ne.jp) was a pain, and thanks to [Fabian's blog post](https://fmentzer.github.io/posts/2020/dictionary/), I actually found it is easier to reverse-engineer Apple's dictionaries, which are also a more authorative source.

## Prerequisites

- python 3.9.5 with lxml 4.6.3
- rustc (cargo) 1.52.1

Haven't tested with lower versions, but you can try.

## Build

Prefer building on Linux. The **lxml** library on MacOS may fail for no reason.

1. Extract raw dictionary data (in `raw/`) into json, stored in `extract/`.

   ```
   mkdir extract && python extract.py
   ```

2. Convert json into the bincode format, by which Rust programs can decode efficiently

   ```
   cd conv && cargo run && cd ..
   ```

3. Build the python module

   ```
   cd jisho
   cargo build --release
   cd ..
   ```

4. Copy or link the dynamic library to the directory where Python can `import jisho`

   ```
   ln -s jisho/target/release/libjisho.so jisho.so
   # libjisho.dylib on MacOS
   ```

5. Now you can `import jisho`, or `import pyjisho`, which is a higher-level wrapper. See `test.py` for example usage and run `python test.py` to check example output.

## Sample Output

See `out/result.html`, which should be the result of running `python test.py`

![Sample output](./img/safari.png)

The styles do not look exactly the same in all browsers, because CSS attributes that are specific to Apple systems are extensively used. I may fix the CSS issues later.

# TODO

- rewrite in Rust
  - why does flate2's zlib decoder behave differently from Python's?
  - what crate is comparable to lxml?
