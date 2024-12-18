
use std::{collections::HashMap, io};

use crate::hard_drive::HDD;
pub trait FileSystem<T>{
    fn get_file(&self, filename: &String) -> Result<Vec<&T>, io::Error>;
    fn write_file(&mut self, filename: &String, content: Vec<T>) -> Result<(), io::Error>;
    fn free_file(&mut self, filename: &String) -> Result<(), io::Error>;
}

pub struct LinkedAllocation<T>{
    hard_drive: HDD<(usize, T)>,
    directory: HashMap<String, usize>
}

impl<T: Clone> LinkedAllocation<T>{
    pub fn new(hard_drive: HDD<(usize, T)>) -> Self{
        let directory: HashMap<String, usize> = HashMap::new();
        LinkedAllocation{
            hard_drive,
            directory
        }
    }
}

impl<T: Clone> FileSystem<T> for LinkedAllocation<T>{

    fn get_file(&self, filename:&String) -> Result<Vec<&T>, io::Error>{
        /*
         * Traverse the linked list to read the file. Return a vector with references to each block.
         */
        if !self.directory.contains_key(filename){
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("{filename} does not exist.")));
        }
        let mut data = Vec::new();
        let mut idx = self.directory.get(filename).unwrap();
        loop{
            if *idx == self.hard_drive.get_capacity(){break;}
            let (new_idx, block) = self.hard_drive.read(*idx);
            data.push(block);
            idx = new_idx;
        }
        Ok(data)
    }

    fn write_file(&mut self, filename: &String, content: Vec<T>) -> Result<(), io::Error>{
        // content is a list of blocks
        // check that the file does not already exist
        if self.directory.contains_key(filename){
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("{filename} already exists.")));
        }
        // check that there is sufficient space
        if self.hard_drive.get_free_blocks() < content.len(){
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Not enough space")));
        }
        // find free blocks and write (in reverse)
        let mut next_block = self.hard_drive.get_capacity();
        for value in content.iter().rev(){
            let block_idx = self.hard_drive.find_free_block().unwrap();
            let (ptr, body) = self.hard_drive.allocate(block_idx).unwrap();
            // set the pointer to next block
            (next_block, *ptr) = (block_idx, next_block);
            // fill the body
            *body = value.clone();
            // set this address for the next thing
        }
        self.directory.insert(filename.clone(), next_block);
        Ok(())

    }

    fn free_file(&mut self, filename: &String) -> Result<(), io::Error>{
        /*
         * Traverse the linked list and free as we go.
         */
        if !self.directory.contains_key(filename){
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("{filename} does not exist.")));
        }
        let mut idx = *self.directory.get(filename).unwrap();
        loop{
            if idx == self.hard_drive.get_capacity(){break;}
            let (new_idx, _) = self.hard_drive.read(idx.clone()).clone();
            self.hard_drive.free(idx.clone());
            idx = new_idx;
        }
        self.directory.remove(filename);
        Ok(())
    }

}

pub struct IndexedAllocation<T>{
    hard_drive: HDD<T>,
    directory: HashMap<String, Vec<usize>>
}

impl<T: Clone> FileSystem<T> for IndexedAllocation<T>{
    fn get_file(&self, filename: &String) -> Result<Vec<&T>, io::Error> {
        /*
         * Traverse the linked list to read the file. Return a vector with references to each block.
         */
        todo!()
    }

    fn write_file(&mut self, filename: &String, content: Vec<T>) -> Result<(), io::Error> {
        todo!()
    }

    fn free_file(&mut self, filename: &String) -> Result<(), io::Error> {
        todo!()
    }
} 