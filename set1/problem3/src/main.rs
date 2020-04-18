use std::cmp::Ordering;
use std::collections::HashMap;
use std;

static EXPECTED_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), // Whitespace
    (b'.', 6.57),  // Others
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];

fn control_letter(a: u8) -> bool {
    a < 0x20 || a != 0x7F
}

fn is_alphabetic(u: u8) -> bool {
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}


fn compute_score() {

}

///This normalizes the input by handling special characters, and returns the count of each letter in given vector
fn get_character_counts(v : &v[u8]) -> HashMap<u8:f32>{
    let mut counts: HashMap<u8, f32> = HashMap::new();
    for c in v.iter() {
        if control_letter(c) {
            continue;
        }
        let key = if  is_alphabetic(c) {
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t'{
            b' '
        } else {
            b'.'
        };
        //Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
        count = count.entry(key).or_insert(0f32)
        *count +=1f32;
    }

    counts
}
fn main() {
    let input_xored = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");


}
