use crate::file;
static BLOCK_SIZE : u32 = 1024; //每个数据块的大小为1kb

//part 1
pub struct Boot_Block {
    
}

//part 2
pub struct Block_Group {
    superblock : SuperBlock, //超级块
    group_descriper_table : Group_Descriper_Table,//块组描述
    inode_table : Inode_Table, // inode表，用于描述数据块
    inode_bitmap : Vec<u8>, // inode位图,判断inode是否分配
    block_bitmap : Vec<u8>, // 块位图// 用于标识区块组中所有区块的使用状态
    data_block : Vec<Data_Block>
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

struct Inode_Table {
    i_mode : u16,
    i_size : u32,
    i_atime : u32,
    i_ctime : u32,
    i_mtime : u32,
    i_dtime : u32,

}


struct Data_Block {

}

enum Permission {
    Read = 1,
    Write = 2,
    Excute = 4 //可执行
}
struct inode {    //node块的指针
    file_type : file::FileType,
    size : u64,
    permission : u8,//三位代表三个权限
    atime : u32, //最后访问时间
    ctime : u32, //最后inode修改时间
    mtime : u32, //最后修改时间
    direct_pointer : u32, //直接索引
    singly_indirect_block : u32, //一级索引
    doubly_indirect_block : u32,
    triply_indirect_block: u32,
}

fn main() {

}



