use std::{time::{SystemTime, UNIX_EPOCH}, vec, io::{Bytes, SeekFrom}};
use bincode;
use serde::de::value::SeqDeserializer;

use crate::{file::{self, DirectoryEntry}, block};
const BLOCK_SIZE : usize = 1024; //每个数据块的大小为1kB
//一个块的大小控制了，inode和bitmap的位图的大就是一个块的大小
//那么块的数据也就确定了，1024*8
const BLOCK_COUNT : usize = BLOCK_SIZE * 8;
//part 1
pub struct Boot_Block {
    
}

//part 2
pub struct Block_Group {
    superblock : SuperBlock, //超级块
    pub group_descriper_table : Group_Descriper_Table,//块组描述
    inode_bitmap : Vec<Option<u8>>, // inode位图,判断inode是否分配
    block_bitmap : Vec<Option<u8>>, // 块位图// 用于标识区块组中所有区块的使用状态
    inode_table : Vec<Inode>, // inode表，用于描述数据块 
    data_block : Vec<Data_Block>
}

pub struct SuperBlock{
    s_inodes_count  : u32,   
    s_block_count : u32,
    s_free_blocks_count : u32,
    s_free_inodes_count : u32,
    s_mtime : u32,//最后一次修改
    s_wtime : u32,//最后一次写入
}


pub struct Group_Descriper_Table {
    bg_block_bitmap : u32,
    bg_inode_bitmap : u32,
    bg_inode_table : u32,
    bg_free_block_count : u16,
    bg_free_inode_count : u16,
    bg_used_dirs_count: u16,
}

struct Inode{
    i_mode : u16,//16位置用于表示文件的类型和权限
    i_size : u32,
    i_atime : u32,//存储的是秒数
    i_ctime : u32,
    i_mtime : u32,
    i_dtime : u32,
    i_block: [Option<u32>;15],//指向的是数据块
    //file_type: FileType,可有可无
    pub direct_pointer : [Option<u32>;12], //直接索引，前12个块 Some是指向数字的指针，要么就是None
    pub singly_indirect_block : Option<u32>, //一级索引，第13个块
    pub doubly_indirect_block : Option<u32>, //二级索引，第14个块
    pub triply_indirect_block: Option<u32>, //二级索引，第15个块
}

struct Data_Block {
    pub data:[u8; BLOCK_SIZE as usize],//每个文件块的大小为1kB
}


impl Block_Group {
    pub fn  new() -> Self{
        Block_Group{
            superblock:SuperBlock::new(),
            group_descriper_table : Group_Descriper_Table::new(),
            inode_bitmap : vec![],
            block_bitmap : vec![],
            inode_table : vec![],
            data_block : vec![]

        }
    }
    pub fn list(&self) {
        for block in &self.data_block {
                let file_name =  block.get_file().name;
                print!("{} ",file_name);
        }
    }
    //找到一个空的inode块 元组表示第几个块的第几个指针
    fn get_inode(&self) -> (i32,i32){
        let mut index = 0;
        for inode_index in self.inode_bitmap {
            if let Some(_) = inode_index {
                index += 1;
            }
            else {
                let num = self.inode_table[index as usize].get_index();
                if num != -1 {
                    return (index,num);
                }
                //找二级指针的值,todo 
                else {

                }
            }
        }
        (-1,-1)
    }    
    fn get_inode_pointer() {


    }
    //找到空的块
    pub fn get_block(&self) ->i32 {
        let mut index = 0 ;
        for block_index in self.block_bitmap {
            if block_index == None {
                return index;
            }
            index += 1;
        }
        0
    }
    //将文件写入数据块中
    fn write_to_block(&self,dir : DirectoryEntry){
        let dir_data = bincode::serialize(&dir).unwrap();//把数据序列化
        let (i_index,index) = self.get_inode();//知道写入到哪个inode中
        let block_index = self.get_block();
        if i_index != -1 {
            if block_index != -1 {
                
            }
            else {
                let mut data_block = Data_Block::new();
                self.data_block.push(data_block);
                self.group_descriper_table.gdb_update(1, 1,1);
                data_block.write(&dir_data)
                

            }
        }
        //需要新建inode
        else{

        }


    }
}




impl SuperBlock {
    pub fn new() -> Self{
        SuperBlock{
            s_inodes_count : BLOCK_COUNT as u32,
            s_block_count  : BLOCK_COUNT as u32,
            s_free_blocks_count : BLOCK_COUNT as u32,
            s_free_inodes_count : BLOCK_COUNT as u32,
            s_mtime : get_current_time(),
            s_wtime : get_current_time(),
        }
    }
}


impl Group_Descriper_Table {
    pub fn new() -> Self {
        Group_Descriper_Table{
            block_bitmap : 2,
            inode_bitmap : 3,
            inode_table : 4,
            free_block_count : BLOCK_COUNT as u16,
            free_inode_count :BLOCK_COUNT as u16,
            dirs_count : 0,
        }
    }
    pub fn gdb_update(&mut self, block_change : i16, inode_change : i16, dir_change : i16){
        self.free_block_count  = (self.free_block_count as i16 + block_change) as u16;
        self.free_inode_count  = (self.free_inode_count as i16 + inode_change) as u16;
        self.dirs_count = (self.dirs_count as i16 + dir_change) as u16;
    }
}
enum Permission {
    Read = 0x0100,
    Write = 0x0080,
    Excute = 0x0040 //可执行        
}
//枚举不支持｜和&，先转化为u16
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


impl Inode {
    fn new() -> Self {
        Inode{
            i_mode : 0x0000,
            i_size : 0,
            i_atime : get_current_time(),
            i_ctime : get_current_time(),
            i_mtime : get_current_time(),
            i_dtime : get_current_time(),
            i_block : Default::default(),//初始化为15个None的数组
            direct_pointer : [None; 12],
            singly_indirect_block : None,
            doubly_indirect_block : None,
            triply_indirect_block : None,
        }
    }   
    //找到第一个空的直接指向的指针
    pub fn get_index(&self) -> i32 {
        for i in self.direct_pointer{
            if i == None{
                return i.unwrap() as i32;
            }
        }
        -1 
    }

}



impl Data_Block {
    pub fn get_file(&self) -> file::DirectoryEntry{
        bincode::deserialize(&self.data).unwrap()
    }
    pub fn new() -> Self {
        Data_Block{
            data:[0; BLOCK_SIZE as usize]
        }
    }
    pub fn write(&mut self,data: &[u8]) {
        for (i, &byte) in data.iter().enumerate().take(BLOCK_SIZE) {
            self.data[i] = byte;
        } 
    }
    
}

//usr access tight


fn get_current_time() -> u32 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration.as_secs() as u32

}
