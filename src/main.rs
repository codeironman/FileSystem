mod file;
mod disk;
use std::io::{stdout, Write, stdin};
mod test;
use disk::EXT2FS;
mod block;
use fuser::MountOption;
use crate::block::Boot_Block;


fn main() {
    let mountpoint = "";
    let boot = Boot_Block{};
    let name= String::new();
    let pw = String::new();
    
    let mut filesystem = EXT2FS::new(name,pw);
    // let fs = EXT2FS::new(name,pw,boot);
    //fuser::mount2(fs, mountpoint, &[MountOption::FSName("myfs".to_string())]).unwrap();

    loop {
        print!("% >");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let input = s.trim();
        println!("{}",input);
        match input {
            "q" => break,
            "ls" => 
        }
        if input == "q" {
            break;
        }
    
    }
}
