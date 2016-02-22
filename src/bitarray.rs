pub struct BitArray {
    array: [usize; 64000]
}

impl BitArray {
    pub fn new() -> BitArray {
        const SIZE: usize = 64000; 
        /*println!{"Creating new bitarray of size {}/64 = {} usizes",n, size}*/
        //println!("{:?}, in binary: {:b})",vec, vec[0]);
        let vec: [usize; SIZE] = [0usize.wrapping_sub(1); SIZE];
            BitArray {
                array: vec
                }
    }
    pub fn set(&mut self, index: usize, bit:  bool){
/*        println!("setting {} to {}",index, bit);
        println!("array index: {} inner location: {}", index/64, index%64);
        println!("array len: {}", self.array.len());*/
    
        match bit{
            true=>  {self.array[index/64] |= 1 << index%64},
            false=> self.array[index/64] &= !(1 << index%64)
        
        }    
    } 

    pub fn get(&mut self, index: usize) -> bool {
/*        println!("getting {}",index);
        println!("array index: {} inner location: {}", index/64, index%64);
        println!("array len: {}", self.array.len());*/
       return self.array[index/64] & (1 <<index%64) !=0;
    }


}

