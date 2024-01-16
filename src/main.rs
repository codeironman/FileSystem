mod disk;
mod file;
use std::io::{stdin, stdout, Write};
mod test;
use disk::EXT2FS;
mod bitmap;
mod block;
use env_logger;
use fuser::{Filesystem, MountOption};
use log::{info, warn};

fn main() {
    let mountpoint = "/Users/caofengyi/code/os/mount";
    let name = "123".to_string();
    let pw = "abc".to_string();
    let filesystem = EXT2FS::new(name, pw);
    info!("Begin to load our filesystem");
    fuser::mount2(
        filesystem,
        mountpoint,
        &[MountOption::FSName("myfs".to_string())],
    )
    .unwrap();

    // loop {
    //     print!("% >");
    //     let _ = stdout().flush();
    //     let mut s = String::new();
    //     stdin().read_line(&mut s).unwrap();
    //     let input = s.trim();
    //     let mut parts = input.split_whitespace();
    //     let command = parts.next().unwrap_or("");
    //     let args = parts.collect::<Vec<&str>>();
    //     println!("{}",input);
    //     // match input {
    //     //     "q" => break,
    //     //     "ls" => filesystem.ls(0),
    //     //     "mkdir" => {
    //     //         filesystem.mkdir(req, parent, name, mode, umask, reply)
    //     //     }
    //     //     "rmdir" => {
    //     //         if let Some(dir_name) = args.first() {
    //     //             match filesystem.rmdir(dir_name) {
    //     //                 Ok(_) => println!("Directory removed"),
    //     //                 Err(e) => println!("Error removing directory: {}", e),
    //     //             }
    //     //         } else {
    //     //             println!("Usage: rmdir <directory>");
    //     //         }
    //     //     },
    //     //     &_ => todo!()
    //     // }
    // }
}
