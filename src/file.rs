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


struct File{
    inode : u32,
    file_size :u16,
    name_len : u8,
    file_type :FileType,
    name : String
}

struct inode {
    file_type : FileType,
    size : u64,
    atime : u32, //最后访问时间
    ctime : u32, //最后inode修改时间
    mtime : u32, //最后修改时间
    direct_pointer : u32, //直接索引
    singly_indirect_block : u32, //一级索引
    doubly_indirect_block : u32,
    triply_indirect_block: u32,
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




fn main(){
    println!("YES");
}