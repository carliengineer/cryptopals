use hex;
use xor::XOR;

fn main() {
    let first_hex = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let second_hex = hex::decode("686974207468652062756c6c277320657965").unwrap();

    let result = &first_hex.xor(&second_hex);
        println!("{:?}", hex::encode(result));

}


// pub trait Xor {
//     fn xor(&self, _ :&Self) -> Vec<u8>;
//     fn xor_inplace(&mut self, _ :&Self);
// }   

// impl Xor for [u8] {
//     fn xor(&self, t : &[u8]) -> Vec<u8>{
//         //copies self into a new vec
//         let mut result = self.to_vec();
//         result[..].xor_inplace(t);
//         result
//     }

//     fn xor_inplace(&mut self, t: &[u8]) {
//         //Returns an iterator over chunk_size elements of the slice at a time, starting at the beginning of the slice.
//         for chunk in self.chunks_mut(t.len()){
//             let len = chunk.len();
//             for (c, &d) in chunk.iter_mut().zip(t[..len].iter()) {
//                 *c ^= d;
//             }
//         }    
//     }
// }
