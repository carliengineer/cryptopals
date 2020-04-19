// use std::cmp::Ordering;
use std::collections::HashMap;
use std;
use std::collections::BTreeMap;
use xor::XOR;
  
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
    a < 0x20 || a == 0x7F
}

fn is_alphabetic(u: u8) -> bool {

    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
   // (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}

///This normalizes the input by handling special characters, and returns the count of each letter in given vector
fn get_character_counts( v : &[u8]) -> HashMap<u8, f32>{
    println!("INPUT {:?}", v);
    let mut char_counts: HashMap<u8, f32> = HashMap::new();
    for &c in v.iter() {
        //println!("letter:{:?}", c);  
        if control_letter(c) {
            continue;
        }
        let key = if  is_alphabetic(c) {
           // println!("ALPHA");
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t'{
            b' '
        } else {
            //println!("SOMETHING ELSE");
            b'.'
        };
       // println!("key: {:?}", key);
        //Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
        let count = char_counts.entry(key).or_insert(0f32);
        *count +=1f32;
        //println!("key: {:?}", key);
    }
    char_counts

}

pub fn compute_score(v: &[u8]) -> u32 {
    //checks if all bytes in the vec are in ascii range
    if !v.is_ascii() {
        return std::u32::MAX;
    }

    if v.iter().any(|&c| control_letter(c) && c != b'\n') {
        return std::u32::MAX;
    }

    let counts = get_character_counts(v);
    let length = v.len() as f32;
    //1) initial value(0f32), 2)
    EXPECTED_FREQUENCIES.iter().fold(0f32, |a, &(c, score)| {
        let expected_count = (score / 100f32) * length;
        let &actual_count = counts.get(&c).unwrap_or(&0f32);
        a + (expected_count - actual_count).powi(2)
    }) as u32
}

fn single_char_xor(v : &[u8]) -> u8{
    //arbitrary bytes
    (0u8..=255)
        .min_by_key(|&u| compute_score(&v.xor(&[u])))
        .unwrap() // unwrap is ok
}

fn main() {
    let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    println!("{:?}", input);

    let mut score : f32 = 0f32;
    let key = single_char_xor(&input);
    println!("asd {:?}", key);
    let output =  String::from_utf8(input.xor(&[key])).unwrap();
    println!("asd {:?}", output);
}
