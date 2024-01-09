use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize)]
enum FileType{
    Unknow = 0,
    Regular = 1,//普通文件
    Directory = 2,//目录文件
    // CharacterDevice = 3,
    // BlockDevice = 4,
    // FIFO = 5,
    // Socket = 6,
    // SymbolicLink = 7,
}
#[derive(Serialize,Deserialize)]//能够派生到自己定义的文件
pub struct DirectoryEntry{
    inode : u32,
    file_size :u16,
    name_len : u8,
    file_type :FileType,//8位二进制
    pub name : String
}




