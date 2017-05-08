# Primo

`primo` is a library and command-line tool to sort files like UNIX’s `sort`.
Unlike `sort`, however, it interprets series of digits as full numbers:

```sh
# input
I have 9 apples.
I have 42 apples.
I have 5 apples.


# sort
I have 42 apples.
I have 5 apples.
I have 9 apples.

# primo
I have 5 apples.
I have 9 apples.
I have 42 apples.
```

Note this is my first ever Rust program so the code might not be the best.

## Usage

### Command-Line

Install with `cargo install primo`.

    primo [<filename>]

It reads on `stdin` or from the provided file and print the sorted version on
stdout.

### Library

```rust
extern crate primo;

fn main() {
    let mut lines = vec![
        "my 1st line".to_string(),
        "...".to_string(),
        "my 15th line".to_string(),
        "my 2nd line".to_string(),
    ];

    primo::sort_vec(&mut lines);

    // prints:
    //    ...
    //    my 1st line
    //    my 2nd line
    //    my 15th line
    //
    for line in lines {
        println!("{}", line);
    }
}
```

## Known Issues

* The sort is quite slow for now because the parsing function is called
  multiple times on each string
* Chars are treated as numbers, so `"abc"` will sort *after*
  `"25bc"` and *before* `"27bc"` because `'a'`’s `i32` value is `26`.

## FAQ

### What about `sort`’s `-V` option?

The GNU `coreutils` package has a `sort` implementation that supports `primo`’s
main use-case with its `-V` option. I unfortunately learnt about this option
after writing `primo`.
