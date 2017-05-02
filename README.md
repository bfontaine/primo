# Primo

`primo` is a command-line tool to sort files like UNIX’s `sort`. Unlike `sort`,
however, it interprets series of digits as full numbers:

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
