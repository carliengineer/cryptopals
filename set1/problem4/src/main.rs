// use std::cmp::Ordering;
use std::collections::HashMap;
use std;
use xor::XOR;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("4.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    
    for (index, line) in reader.lines().enumerate() {
        let line = hex::decode(line.unwrap());
        vec.push(line);
    }
    
    

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
    //1) initial value(0f32), 2)accumulator and element, 3)function
    EXPECTED_FREQUENCIES.iter().fold(0f32, |a, &(c, score)| {
        let expected_count = (score / 100f32) * length;
        let &actual_count = counts.get(&c).unwrap_or(&0f32);
        a + (expected_count - actual_count).powi(2)
    }) as u32
}

fn single_char_xor(v : &[u8]) -> u8{

     v.iter().fold(0u8, || {

     }
    );

        // .min_by_key(|&u| 
        //     let mut iter = a.iter();
        //     compute_score(iter.next()))
        // .unwrap() // unwrap is ok
}