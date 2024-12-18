mod hard_drive;
mod file_system;

use hard_drive::HDD;
use file_system::{FileSystem, LinkedAllocation};

fn main() {
    let hardware: HDD<(usize, u8)> = HDD::new(10, (0, 0));
    let mut filesystem = LinkedAllocation::new(hardware);
    // expect a failure
    let result = filesystem.get_file(&String::from("test"));
    match result{
        Ok(_) => println!("FAILURE: This should NOT have succeeded"),
        Err(e) => println!("SUCESS: This was not supposed to succeed. Error: {}", e)
    }
    // try to write a file
    let result = filesystem.write_file(&String::from("test"), vec![1, 2, 3, 4, 5]);
    match result{
        Ok(_) => println!("SUCCESS: File written"),
        Err(e) => println!("FAILURE: File not written. Error: {}", e)
    }
    // now read the content
    let result = filesystem.get_file(&String::from("test"));
    match result{
        Ok(data) => {
            println!("SUCCESS: File read. {} bytes", data.len());
            for block in data.iter(){
                println!("{}", block);
            }
        },
        Err(e) => println!("FAILURE: File not read. Error: {}", e)
    }
    // now free the file
    let result = filesystem.free_file(&String::from("test"));
    match result{
        Ok(_) => println!("SUCCESS: File freed"),
        Err(e) => println!("FAILURE: File not freed. Error: {}", e)
    }
    // now try to read it again. It should fail
    let result = filesystem.get_file(&String::from("test"));
    match result{
        Ok(_) => println!("FAILURE: This should NOT have succeeded"),
        Err(e) => println!("SUCESS: This was not supposed to succeed. Error: {}", e)
    }
    // try to free something that does not exist
    let result = filesystem.free_file(&String::from("test"));
    match result{
        Ok(_) => println!("FAILURE: This should NOT have succeeded"),
        Err(e) => println!("SUCESS: This was not supposed to succeed. Error: {}", e)
    }
    // try to allocate a file that is too big
    let result = filesystem.write_file(&String::from("test"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    match result{
        Ok(_) => println!("FAILURE: This should NOT have succeeded"),
        Err(e) => println!("SUCESS: This was not supposed to succeed. Error: {}", e)
    }
    // allocate the entire hard drive then try to write a file
    let r1 = filesystem.write_file(&String::from("test1"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    match r1{
        Ok(_) => println!("SUCCESS: File written"),
        Err(e) => println!("FAILURE: File not written. Error: {}", e)
    }
    let result = filesystem.write_file(&String::from("test2"), vec![1]);
    match result{
        Ok(_) => println!("FAILURE: This should NOT have succeeded"),
        Err(e) => println!("SUCESS: This was not supposed to succeed. Error: {}", e)
    }

}
