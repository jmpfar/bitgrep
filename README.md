# bitgrep

It's grep for data types. Ever found yourself looking for a specific numerical value/range in a heap of binary files?

Now you can!

Useful for DFIR, security research and general debugging work, especially when you know what you're looking for but don't know where.

## Install

Use `cargo install` to install the binary from crates.io:

```console
$ cargo install bitgrep
$ bitgrep --data-type u32 --file data.raw -m 55 -M 144
```

Alternatively you can build a binary using the code from github:

```console
$ git clone https://github.com/jmpfar/bitgrep.git
$ cd bitgrep
$ cargo build --release
$ target/release/bitgrep --data-type f64 --file data.raw -m 29.15 -M 36.0
```

## Usage

To find all all the doubles (`f64`) with values `29.15 <= x <= 36.0`:

```console
$ bitgrep --data-type f64 --file data.raw -m 29.15 -M 36.0

./data.raw: [0x16B6] f64: 34.415624980210914 [9b483333354140]
./data.raw: [0xFDBB] f64: 30.215716721498428 [3d983639373e40]
```

The output format is:

```console
file_path: [offset] data_type: value [value_in_hex]
```

### Options

In order to find a single literal value you can use the `--literal` or `-l` flag.
Float comparison is approximate with a [ULPS](https://en.wikipedia.org/wiki/Unit_in_the_last_place) of 4 (will be configurable in the future):

```console
$ bitgrep --data-type f64 --file data4.raw --literal 29.15385732 \
    --endian big
```

You can also filter by [entropy](<https://en.wikipedia.org/wiki/Entropy_(information_theory)>) to remove values that have a high chance of being noise.

Entropy ranges between 0 and 8 where 8 represents random data. Entropy greater than 7.5 is usually encrypted, compressed or random. English text has a value of between 3.5 and 5.

```console
$ bitgrep --data-type i128 --file data.raw --literal 123 \
    --max-entropy 7.5
```

You can use a pipe with the special `-` file path:

```console
$ cat data.raw | bitgrep --data-type u8 --file - --literal 3
```

To reduce noise in binary files that contain zero bytes, you can use `--exclude-zero`. This excludes all absolute zero values (`0x0`)

```console
$ bitgrep --data-type i32 --file data.raw --min -30 --max 30 \
    --exclude-zero
```

The above command does not filter values that are approximately close to zero (e.g. `0.00000000000000001`). This might be useful when reducing noise in floating point searches. Alternatively use:

```console
$ bitgrep --data-type f64 --file data.raw --min -30.0 --max 30.0 \
    --exclude-literal 0.0
```

Currently there is no native support for directory globbing or recursion, if you need to search multiple files you can use the `find` command:

```console
$ find . -type f -exec bitgrep \
		--data-type i32 --file {} --max -78 --min -83 \
		--endian little \;
```

### Supported Types

Currently bitgrep supports all rust numeric data types (use with `--data-type`):

| Rust | C                   |
| ---- | ------------------- |
| i16  | short               |
| i32  | int                 |
| i64  | long long           |
| i128 | \_\_int128 (GCC)    |
| u16  | unsigned short      |
| u32  | unsigned int        |
| u64  | unsigned long long  |
| u128 | unsigned \_\_int128 |
| f32  | float               |
| f64  | double              |

## TODO

> [!WARNING]  
>  Everything below this point does not exist yet!

Feel free to send pull requests, hopefully I'll get to these before 2026

1. [x] Filter files by [entropy](<https://en.wikipedia.org/wiki/Entropy_(information_theory)>)
2. [x] Add pipe support and other unix semantics
3. [x] Use stderr
4. [ ] Color output
5. [ ] Hex dump output
6. [x] Literals search
7. [ ] Hex search (e.g. `0AAD[33-4A]DF`)
8. [x] Exclude zeros
9. [x] Exclude approximate literal values
10. [ ] Sane error messages
11. [ ] Exclude extreme exponent values
12. [ ] Binary releases
13. [ ] Recursive file search / glob
14. [ ] Date types
    1. [ ] 32-bit/64-bit Unix epoch (milliseconds, microseconds, seconds)
    2. [ ] Windows
       1. [ ] FILETIME
       2. [ ] SYSTEMTIME
       3. [ ] OLE automation
       4. [ ] CLR Time
    3. [ ] Apple timestamps
15. [ ] String Search
    1. [ ] UTF-8
    2. [ ] UTF-16
    3. [ ] ASCII code pages
    4. [ ] Search string representations of number range: e.g. "10.2" .. "10.722"
    5. [ ] Regex
16. [ ] Performance improvements
    1. [ ] Convert to static dispatch
    2. [ ] Search without converting bytes to number
17. [ ] Rule engine, see below
18. [ ] Misc
    1. [ ] GUIDs
    2. [ ] IP addresses
    3. [ ] Custom structs
19. [ ] Debt
    1. [ ] Refactor printing to different object/trait
    2. [ ] Add integration tests

### Rule engine

TODO: An imagined JSON of a rules file that can be used as a search configuration.

```json
{
  "filters": {
    "file": {
      "magic": "0xABDEF",
      "types": [
        {
          "double": { "min": 80.3432, "max": 82.221112, "exclude-zero": true }
        },
        { "double": { "min": -32.865, "max": 31.53221, "exclude-zero": true } },
        { "string": { "literal": "AMAzING" } },
        { "string": { "regex": "12334+" } },
        { "bytes": { "literal": "0xDEADBEEF" } },
        { "integer": { "min": -10, "max": 12, "as_string": true } }
      ],
      "entropy": {
        "max": 6
      }
    }
  }
}
```

The idea is to have predefined rules for specific scenarios and some level of boolean operators for better filtering.

For example, get me all IPs in binary or string form in the ranges `192.168.1.0 - 192.168.3.255` or `10.0.0.1 - 10.0.30.255`
