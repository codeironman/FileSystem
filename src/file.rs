pub enum FileType{
    Unknow,
    Regular,
    Directory,
    // CharacterDevice,
    // BlockDevice,
    // FIFO,
    // Socket,
    // SymbolicLink,
}


pub struct File{
    inode : u32,
    file_size :u16,
    name_len : u8,
    file_type :FileType,
    name : String
}




impl File{
    fn new(name : String, file_type : FileType) -> Self{
        File{
            inode : 0,
            file_size : 0,
            name_len : name.len() as u8,
            file_type : file_type,
            name : name

        }
    }
}


