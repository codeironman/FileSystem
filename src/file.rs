use std::iter::StepBy;

use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize)]
pub enum FileType{
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
    inode : u32,//指的是这个目录文件指向的inode
    file_size :u16,//按照
    name_len : u8,
    file_type :FileType,//8位二进制
    pub name : String
}


impl DirectoryEntry {
    pub fn new(file_name : String, fild_type : FileType,index_node : u32, _size : u16) -> Self{
        DirectoryEntry{
            name :file_name,
            file_type : fild_type,
            name_len : file_name.len() as u8,
            inode :index_node,
            file_size : _size
        }
    }

}




