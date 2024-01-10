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


impl DirectoryEntry {
    fn new(&mut self,file_name : String, fild_type : FileType,index_ndoe : u32, size : u16) -> DirectoryEntry{
        DirectoryEntry{
            name :file_name,
            file_type : fild_type,
            name_len : self.name.len() as u8,
            inode :index_ndoe,
            file_size : size
        }
    }

}




