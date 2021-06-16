# Reverse-engineering Apple's JP, JP-EN and JP-CN Dictionaries

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

   This may require up to 20GB of RAM. If your RAM is not large enough `cargo build` will crash. You can try `cargo build --debug` instead, which uses less memory.

4. Copy or link the dynamic library to the directory where Python can `import jisho`

   ```
   ln -s jisho/target/release/libjisho.so jisho.so
   # libjisho.dylib on MacOS
   # jisho/target/debug/libjisho.so if `cargo build --debug`
   ```

5. Now you can `import jisho`. See `test.py` for example usage and run `python test.py` to check example output.
