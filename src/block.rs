mod file;

//part 1
struct Boot_Block {

}

//part 2
struct Block_Group {
    superblock : SuperBlock, //超级块
    block_group_descriper : Block_Group_Descriper, //块组描述
    inode_table : Inode_Table, // inode表
    inode_bitmap : Inode_Bitmap, // inode位图
    block_bitmp : Block_Bitmap, // 块位图

}

struct SuperBlock{
    inodes_num : u32,
    
}

struct Block_Group_Descriper {

}

struct Inode_Table {

}

struct Inode_Bitmap {

}

struct Block_Bitmap {

}




