# bitgrep

It's grep for data types. Ever found yourself looking for a specific numerical value/range in a heap of binary files?

Now you can! 

Useful for DFIR, security research and general debugging work, especially when you know what you're looking for but don't know where.

## Usage

You can run the CLI binary either through `cargo run`:

```bash
$ cargo run -- --data-type f64 --file data.raw -m 29.15 -M 36.0
```

The above command finds all the doubles (`f64`) with values `29.15 <= x <= 36.0`.

Alternatively, you can build a binary:

```bash
$ git clone https://github.com/jmpfar/bitgrep.git
$ cd bitgrep
$ cargo build --release
```

and then run the binary:

```bash
$ target/release/bitgrep --data-type f64 --file data.raw -m 29.15 -M 36.0
```

In order to find a single literal value you can use the `--literal` or `-l` flag. 
Float comparison is approximate with a [ULPS](https://en.wikipedia.org/wiki/Unit_in_the_last_place) of 4 (will be configurable in the future):

```bash
$ cargo run -- --data-type f64 --file data4.raw --literal 29.15385732 --endian big
```

You can also filter by entropy to remove values that have a high chance of being noise. Entropy of >7.5 is usually compressed
or encrypted data:

```bash
$ bitgrep --data-type i128 --file data.raw --literal 123 --max-entropy 7.5
```

Currently there is no native support for directory globbing or recursion, if you need to search multiple files you can use the `find` command:

```bash
$ find . -type f -exec /path/to/bitgrep \ 
		--data-type i32 --file {} --max -78 --min -83 --endian little \;
```


### Supported Types

Currently bitgrep supports all rust numeric data types (use with `--data-type`):


| Rust | C                 |
|------|-------------------|
| i16  | short             |
| i32  | int               |
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

Feel free to send pull requests, hopefully I'll get to these before 2026

1. [x] Filter files by [entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory))
2. [ ] Add pipe support and other unix semantics
3. [ ] Use stderr
4. [ ] Color output
5. [ ] Hex dump output
6. [x] Literals search
7. [ ] Hex search (e.g. `0AAD[33-4A]DF`)
8. [ ] Exclude zeros and special valus (`NaN`, Infinty)
9. [ ] Sane error messages
10. [ ] Recursive file search / glob
11. [ ] Date types
    1. [ ] 32-bit/64-bit Unix epoch (milliseconds, microseconds, seconds)
    2. [ ] Windows
       1. [ ] FILETIME
       2. [ ] SYSTEMTIME
       3. [ ] OLE automation
       4. [ ] CLR Time
    3. [ ] Apple timestamps
12. [ ] String Search
	 1. [ ] UTF-8
	 2. [ ] UTF-16
	 3. [ ] ASCII code pages
	 4. [ ] Search string representations of number range: e.g. "10.2" .. "10.722"
	 5. [ ] Regex
13. [ ] Performance improvements
	 1. [ ] Convert to static dispatch
14. [ ] Rule engine, see below
15. [ ] Misc
    1. [ ] GUIDs
    2. [ ] IP addresses
    3. [ ] Custom structs
    4. [ ] Refactor printing to different object/trait


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
