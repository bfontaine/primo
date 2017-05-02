extern crate unicode_segmentation;
use self::unicode_segmentation::UnicodeSegmentation;

fn numerical_string_key(s : &String) -> Vec<i32> {
    // need to decide what we want here
    let k : Vec<i32> = Vec::with_capacity(s.len());

    for graphem in UnicodeSegmentation::graphemes(&**s, true) {
        for ch in graphem.chars() {
            // TODO detect numbers
        }
    }
    k
}

pub fn sort_vec(v : &mut Vec<String>) {
    // TODO
    v.sort_by_key(numerical_string_key);
}
