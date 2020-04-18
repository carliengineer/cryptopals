// /Keyword Self
//The implementing type within a trait or impl block, or the current type within a type definition.

pub trait XOR {
    fn xor(&self, _ :&Self) -> Vec<u8>;
    fn xor_inplace(&mut self, _ :&Self);
}   

impl XOR for [u8] {
    fn xor(&self, t : &[u8]) -> Vec<u8>{
        //copies self into a new vec
        let mut result = self.to_vec();
        result[..].xor_inplace(t);
        result
    }

    fn xor_inplace(&mut self, t: &[u8]) {
        //Returns an iterator over chunk_size elements of the slice at a time, starting at the beginning of the slice.
        for chunk in self.chunks_mut(t.len()){
            let len = chunk.len();
            for (c, &d) in chunk.iter_mut().zip(t[..len].iter()) {
                *c ^= d;
            }
        }    
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
