extern crate unicode_segmentation;
use self::unicode_segmentation::UnicodeSegmentation;

// FIXME unfortunately this is an expensive function and Rust's sort_by_key
// calls it multiple times:
//  https://github.com/rust-lang/rust/issues/34447
//
// We may want to do a Schwartzian-like transform but it needs to allocate
// memory.
fn numerical_string_key(s : &String) -> Vec<i32> {
    // need to decide what we want here
    let mut k : Vec<i32> = Vec::with_capacity(s.len());

    let mut current_number : i32 = 0;
    let mut in_a_number = false;

    for graphem in UnicodeSegmentation::graphemes(&**s, true) {
        let chars : Vec<char> = graphem.chars().collect();
        let size = chars.len();

        // Not sure if that could happen.
        if size == 0 {
            k.push(0);
            continue;
        }

        match chars[0].to_digit(10) {
            Some(d) if size == 1 => {
                in_a_number = true;
                current_number = current_number * 10 + d as i32;
            },
            _ => /* non-digit */ {
                if in_a_number {
                    // Note e.g. "a" and "26" would get the same key
                    k.push(current_number);
                    current_number = 0;
                }
                in_a_number = false;
                k.push(chars.iter().fold(0, |sum, ch| sum + *ch as i32));
            },
        };
    }
    if in_a_number {
        k.push(current_number);
    }
    k
}

/**
 * Sort a vector.
 **/
pub fn sort_vec(v : &mut Vec<String>) {
    v.sort_by_key(numerical_string_key);
}
