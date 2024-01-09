use std::ops::IndexMut;
use bincode;

use crate::file;
const BLOCK_SIZE : usize = 1024; //每个数据块的大小为1kB
const BLOCK_NUM :usize = 4096;

//part 1
pub struct Boot_Block {
    
}

//part 2
pub struct Block_Group {

    superblock : SuperBlock, //超级块
    group_descriper_table : Group_Descriper_Table,//块组描述
    inode_table : Vec<Inode>, // inode表，用于描述数据块
    inode_bitmap : Vec<u8>, // inode位图,判断inode是否分配
    block_bitmap : Vec<u8>, // 块位图// 用于标识区块组中所有区块的使用状态
    pub data_block : Vec<Data_Block>
}
impl Block_Group {
    pub fn list(&self) {
        for block in &self.data_block {
                let file_name = block.get_file().name;
                print!("{} ",file_name);
        }
    }
}



struct SuperBlock{
    inodes_count  : u32,   
    block_count : u32,
    free_block_count : u32,
    free_inodes_count : u32,
    mtime : u32,
    wtime : u32,
}

struct Group_Descriper_Table {
    block_bitmap : u32,
    inode_bitmap : u32,
    inode_table : u32,
    free_block_count : u16,
    free_inode_count : u16,
    dirs_count : u16,
}

enum Permission {
    Read = 0x0100,
    Write = 0x0080,
    Excute = 0x0040 //可执行
}
pub enum FileType{
    Unknow = 0x0000,
    Regular = 0x8000,
    Directory = 0x4000,
    // CharacterDevice = 0x2000,
    // BlockDevice = 0x6000,
    // FIFO = 0x1000,,
    // Socket = 0xC000,
    // SymbolicLink = 0xA000,
}

struct Inode{
    i_mode : u16,//16位置用于表示文件的类型和权限
    i_size : u32,
    i_atime : u32,
    i_ctime : u32,
    i_mtime : u32,
    i_dtime : u32,
    i_block: [Option<u32>;15],
    //file_type: FileType,可有可无
    direct_pointer : [u32;12], //直接索引，前12个块
    singly_indirect_block : u32, //一级索引，第13个块
    doubly_indirect_block : u32, //二级索引，第14个块
    triply_indirect_block: u32, //二级索引，第15个块

}


struct Data_Block {
    pub data:[u8; BLOCK_SIZE],//每个文件块的大小为1kB
}

impl Data_Block {
    pub fn get_file(&self) -> file::DirectoryEntry{
        bincode::deserialize(&self.data).unwrap()
    }
    
}

//usr access tight

