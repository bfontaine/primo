# Primo

`primo` is a library and command-line tool to sort files like UNIX’s `sort`.
Unlike `sort`, however, it interprets series of digits as full numbers:

```sh
# myfile.sh
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

    primo <filename>

It reads the provided filename and print its sorted version on stdout.

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
