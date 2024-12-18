use std::io::{self, Error};
/**
 * HDD is a simple wrapper around a Vec<T>, with a capacity trait.
 */
pub struct HDD<T>{
    /**
     * memory: data structure to hold the memory
     * capacity: the number of blocks in the hard drive
     * T: is the data type of a single block
     */
    memory: Vec<T>,
    free_list: Vec<bool>,
    capacity: usize,
    free_blocks: usize
}
impl<T: Clone> HDD<T>{
    pub fn new(capacity: usize, default: T) -> Self{
        let memory: Vec<T> = vec![default.clone(); capacity];
        let free_list: Vec<bool> = vec![true; capacity];
        let free_blocks = capacity;
        HDD{
            memory,
            free_list,
            capacity,
            free_blocks
        }
    }

    pub fn get_capacity(&self) -> usize{
        self.capacity
    }

    pub fn allocate(&mut self, index: usize) -> Result<&mut T, std::io::Error>{
        match self.free_list[index]{
            true => {
                self.free_list[index] = false;
                self.free_blocks -= 1;
                Ok(&mut self.memory[index])
            },
            false => Err(Error::new(io::ErrorKind::AddrNotAvailable, format!("{index} is already allocated.")))
        }
    }

    pub fn read(&self, index: usize) -> &T{
        return &self.memory[index];
    }

    pub fn free(&mut self, index: usize){
        self.free_blocks += 1;
        self.free_list[index] = true;
    }

    pub fn get_free_blocks(&self) -> usize{
        return self.free_blocks;
    }

    pub fn find_free_block(&self) -> Option<usize>{
        if self.free_blocks == 0{
            return None;
        }
        let mut idx = self.free_list.len();
        for (i, &elem) in self.free_list.iter().enumerate(){
            if elem{
                idx = i;
                break;
            }
        }
        Some(idx)
    }

}