use std::iter::StepBy;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    Unknow = 0,
    Regular = 1, //普通文件
    Directory = 2, //目录文件
                 // CharacterDevice = 3,
                 // BlockDevice = 4,
                 // FIFO = 5,
                 // Socket = 6,
                 // SymbolicLink = 7,
}
#[derive(Serialize, Deserialize, Debug)] //能够派生到自己定义的文件
pub struct DirectoryEntry {
    pub inode: u32, //指的是这个目录文件指向的inode
    file_size: u16, //按照
    name_len: u8,
    file_type: FileType, //8位二进制
    pub name: String,
}

impl DirectoryEntry {
    pub fn new(file_name: String, fild_type: FileType, index_node: u32, _size: u16) -> Self {
        DirectoryEntry {
            name: file_name.clone(),
            file_type: fild_type,
            name_len: file_name.len() as u8,
            inode: index_node,
            file_size: _size,
        }
    }
    pub fn to_bytes(&mut self) -> (Vec<u8>, u16) {
        let dir_data = bincode::serialize(&self).unwrap();
        let dir_size = dir_data.len() as u16;
        self.file_size = dir_size;
        (dir_data, dir_size)
    }
    pub fn get_type(&self) -> fuser::FileType {
        match self.file_type {
            FileType::Regular => fuser::FileType::RegularFile,
            FileType::Directory => fuser::FileType::Directory,
            // fuser 不存在 Unknown 这个 type
            FileType::Unknow => fuser::FileType::RegularFile,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
