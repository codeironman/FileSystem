use crate::file::*;
use bincode::deserialize;
struct User{
    username : String,
    password : String,
    u_uid : u8,
    u_gid : u8
}
#[debug]
fn main(){
    let mut dir = DirectoryEntry::new("11".to_string(), FileType::Directory, 0, 2);
    let (dir_data, dir_size) = dir.to_bytes();
    dbg!(dir_data);
}