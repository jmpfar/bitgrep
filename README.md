# bitgrep

It's grep for data types. Ever found yourself looking for a specific double value/range in a heap of binary files?

Now you can! 

Useful for DFIR, security research and general debugging work, especially when you know what you're looking for but don't know where.

## Usage

You can run the CLI binary either through `cargo run`:

```bash
$ cargo run -- --data-type f64 --file data.raw -m 29.15 -M 36.0
```

This finds all the doubles (`f64`) with values 29.15 <= x <= 36.0, use the following command:

Alternatively, you can build a binary:

```bash
$ cargo build
```

and then run the binary:

```bash
$ cd target/debug
$ bitgrep --data-type f64 --file data.raw -m 29.15 -M 36.0
```

In order to find a single literal value you need to use a hack. Use a minimum and maximum that is equal to the value you're looking for:

```bash
$ cargo run -- --data-type i128 --file data4.raw -m 36 -M 36 --endian big
```

Currently there is no native support for directory globbing or recursion, if you need to search multiple files you can use the `find` command:

```bash
$ find . -type f -exec /path/to/bitgrep --data-type i32 --file {} --max -78 --min -83 --endian little \;
```


### Supported Types

Currently bitgrep supports all rust numeric data types (use with `--data-type`):


| Rust | C                 |
|------|-------------------|
| i16  | short             |
| i32  | long              |
| i64  | long long         |
| i128 | __int128 (GCC)    |
| u16  | unsigned short    |
| u32  | unsigned int      |
| u64  | unsigned long     |
| u128 | unsigned __int128 |
| f32  | float             |
| f64  | double            |

## TODO
> [!WARNING]  
>  Everything below this point does not exist yet!

Feel free to send these pull requests, hopefully I'll get to these before 2026

1. Filter files by [entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory))
2. Color output
3. Hex dump output
4. Literals search
5. Hex search (e.g. `0AAD[33-4A]DF`)
6. Recursive file search / glob
7. Date types
   1. 32-bit/64-bit Unix epoch (milliseconds, microseconds, seconds)
   2. Windows
      1. FILETIME
      2. SYSTEMTIME
      3. OLE automation
      4. CLR Time
   3. Apple timestamps
8. String Search
   1. UTF-8
   2. UTF-16
   3. ASCII code pages
   4. Search string representations of number range: e.g. "10.2" .. "10.722"
   5. Regex
9. Performance improvements
   1. Convert to static dispatch
10. Rule engine, see below
11. Misc
    1.  GUIDs
    2.  IP addresses
    3.  Custom structs


### Rule engine
An imagined JSON of a rules file that can be used as a search configuration.

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

The idea is having predefined rules for specific scenarios and some level of boolean operators for better filtering. 

For example, get me only the files that have both an int in range 423..632 and a double in 34.333..37.22.