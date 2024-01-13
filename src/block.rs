use core::num;
use std::{time::{SystemTime, UNIX_EPOCH}, vec, io::{Bytes, SeekFrom}, mem, cmp::min, sync::Arc, ops::Index};
use bincode;
use serde::de::value::SeqDeserializer;

use crate::{file::*, block, bitmap::*};
const BLOCK_SIZE : usize = 1024; //每个数据块的大小为1kB
//一个块的大小控制了，inode和bitmap的位图的大就是一个块的大小
//那么块的数据也就确定了，1024*8
const BLOCK_COUNT : usize = BLOCK_SIZE * 8;
//part 1
pub struct BootBlock {
    
}

//part 2
pub struct BlockGroup {
    superblock : SuperBlock, //超级块
    group_descriper_table : GroupDescriperTable,//块组描述
    inode_bitmap : Bitmap, // inode位图,判断inode是否分配
    block_bitmap : Bitmap, // 块位图// 用于标识区块组中所有区块的使用状态
    inode_table : Vec<Inode>, // inode表，用于描述数据块 
    data_block : Vec<DataBlock>
}
//todo，之后在管理超级块
pub struct SuperBlock{
    s_inodes_count  : u32,   
    s_block_count : u32,
    s_free_blocks_count : u32,
    s_free_inodes_count : u32,
    s_mtime : u32,//最后一次修改
    s_wtime : u32,//最后一次写入
}


pub struct GroupDescriperTable {
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

struct DataBlock {
    pub data:[u8; BLOCK_SIZE as usize],//每个文件块的大小为1kB
}

impl BlockGroup {
    pub fn new() -> Self{
        BlockGroup{
            superblock: SuperBlock::new(),
            group_descriper_table : GroupDescriperTable::new(),
            inode_bitmap : Bitmap::new(BLOCK_SIZE),
            block_bitmap : Bitmap::new(BLOCK_SIZE),
            inode_table : vec![],
            data_block : vec![]

        }
    }
    pub fn full(&self) -> bool{
        self.block_bitmap.free_index() == None
    }

    pub fn bg_list(&self,parent_inode : usize) {
        let mut all_dirs:Vec<DirectoryEntry> = vec![];
        for index in self.inode_table[parent_inode].direct_pointer{
            if let Some(i_block) = index {
                all_dirs.append(&mut self.data_block[i_block as usize].get_all_dirs_name());
            }
        }
        for it in all_dirs {
            print!("{} ",it.name);
        }
    }
    pub fn bg_rmdir(&mut self, parent_inode : usize, name : String){
        for block_index  in self.inode_table[parent_inode].direct_pointer {
            if let Some(index) = block_index {
                self.data_block[index as usize].rmdir_from_data_block(&name);
            }
        }
    }

    pub fn bg_mkdir(&mut self, name : String, parent_inode : usize){
        let child_inode = self.add_entry_to_directory(name, parent_inode);
        self.add_entry_to_directory(".".to_string(), child_inode);
        self.add_entry_to_directory("..".to_string(), child_inode);
    }

    pub fn add_entry_to_directory(&mut self,name :String, parent_inode : usize) -> usize{
        let inode_index = self.get_inode();//分配一个inode
        let mut dir = DirectoryEntry::new(name,FileType::Directory, inode_index as u32, 0);
        let (dir_data,dir_size) = dir.to_bytes();  
        let (block_index,offset,i_block_index) = self.get_block_for_dir(parent_inode, dir_size);//找到写入的块
        self.data_block[block_index].write(&dir_data, offset);
        self.inode_table[parent_inode].inode_update(dir_size as i32,i_block_index,block_index as u32);
        self.inode_bitmap.set(inode_index, true);
        self.block_bitmap.set(block_index, true);
        inode_index
    }
    //找到第一个还有容量的块能够写入文件夹的块，用于写入文件夹
    pub fn get_block_for_dir(&self,inode_index : usize,dir_byte : u16) -> (usize, usize, usize) {
        //先实现直接指针的，todo多级指针
        let mut inode_index = 0;
        for i in self.inode_table[inode_index].direct_pointer {
            if let Some(val) = i{
                let free_bytes = self.data_block[val as usize].count_free_bytes();
                if free_bytes > dir_byte {
                    return (val as usize, BLOCK_SIZE - free_bytes as usize,inode_index);
                }
                inode_index += 1;
            }
            
        }
        (0,0,0)
    }

    pub fn get_inode_index(&mut self,inode_index : usize) -> i32{
        self.inode_table[inode_index].get_index()
    }
    fn get_inode(&mut self) -> usize{
        match self.inode_bitmap.free_index() {
            Some(index) => return index,
            None => {
                self.inode_bitmap.set(self.inode_table.len()-1, true);
                self.inode_table.push(Inode::new());
                self.inode_table.len()
            }
        }
    }    
    //找到空的块
    pub fn get_block_for_file(&mut self) -> u32 {
        if let Some(index) = self.block_bitmap.free_index() {
            return index as u32;
        }
        self.block_bitmap.set(self.data_block.len(), true);
        self.data_block.push(DataBlock::new());
        return self.data_block.len() as u32 ;
    }


    fn bg_update(&mut self,block_index: usize,inode_index : usize){
        
         self.block_bitmap.set(block_index, true);
         self.inode_bitmap.set(inode_index, true);
    }
    // pub fn write_dir(&mut self, dir : DirectoryEntry, parent_inode : usize){
    //     let dir_data = bincode::serialize(&dir).unwrap();//把数据序列化
    //     let dir_size = dir_data.len();//这里得到的是字节数
    //     let (num_block,offset) = ((dir_size + BLOCK_SIZE - 1) / BLOCK_SIZE, dir_size % BLOCK_SIZE);
    //     let inode_index= self.get_inode();//分配了一个inode
    //     for i in 0..num_block {
    //         let start = i * BLOCK_SIZE;
    //         let end = min(start+BLOCK_SIZE, dir_size);
    //         let block_data = &dir_data[start..end];
    //         let block_index = self.write_to_block(block_data);
    //         self.inode_table[inode_index].inode_update(block_data.len() as i32,inode_index, block_index as u32);
    //         self.bg_update(block_index, inode_index);
    //     }
    //     self.group_descriper_table.gdt_update(1, 1,1);

    // }
    //将文件写入数据块中
    pub fn write_to_block(&mut self,data :&[u8],block_index: usize,offset : usize) -> usize{
        self.data_block[block_index as usize].write(&data,offset);
        return block_index as usize;
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

impl GroupDescriperTable {
    pub fn new() -> Self {
        GroupDescriperTable{
            bg_block_bitmap : 2,
            bg_inode_bitmap : 3,
            bg_inode_table : 4,
            bg_free_block_count : BLOCK_COUNT as u16,
            bg_free_inode_count : BLOCK_COUNT as u16,
            bg_used_dirs_count: 0,
        }
    }
    pub fn gdt_update(&mut self, block_change : i16, inode_change : i16, dir_change : i16){
        self.bg_free_block_count  = (self.bg_free_block_count as i16 + block_change) as u16;
        self.bg_free_inode_count  = (self.bg_free_inode_count as i16 + inode_change) as u16;
        self.bg_used_dirs_count = (self.bg_used_dirs_count as i16 + dir_change) as u16;
    }
    pub fn full(&self) -> bool{
        self.bg_free_block_count == 0
    }
}
enum Permission {
    Read = 0x0100,
    Write = 0x0080,
    Excute = 0x0040 //可执行        
}
//枚举不支持｜和&，先转化为u16
pub enum Filetype{
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

    pub fn inode_update(&mut self, size : i32,index : usize, block_index : u32) {
        self.i_size += (size + self.i_size as i32) as u32;
        self.direct_pointer[index] = Option::Some(block_index);
        self.i_ctime = get_current_time();
    }
}



impl DataBlock {
    pub fn get_file(&self) -> DirectoryEntry{
        bincode::deserialize(&self.data).unwrap()
    }
    pub fn new() -> Self {
        DataBlock{
            data:[0; BLOCK_SIZE as usize]
        }
    }
    pub fn write(&mut self,data: &[u8],offset : usize) {
        for (i, &byte) in data.iter().enumerate().take(BLOCK_SIZE) {
            self.data[i] = byte;
        } 
    }
    pub fn count_free_bytes(&self) -> u16{
        let mut used_byte = 0;
        for &byte in self.data.iter() {
            if byte != 0x00 {
            }
            else {
                break;
            }
        }
        return (BLOCK_SIZE - used_byte) as u16;
    }
    pub fn get_all_dirs_name(&self) -> Vec<DirectoryEntry>{
        let mut offset = 0;
        let mut dir_vec : Vec<DirectoryEntry> = vec![];
        while offset < BLOCK_SIZE {
            let file_size : usize = bincode::deserialize(&self.data[4+offset..6+offset]).unwrap();//从第四个字节开始解析2个字节为文件的大小
            if file_size == 0 {break;}
            let dir : DirectoryEntry  = bincode::deserialize(&self.data[offset..offset+file_size]).unwrap();
            dir_vec.push(dir);
            offset += file_size;
        }   
        vec![]
    }

    // pub fn lookup_dir() -> Option<u32>{
    //     let mut offset : usize = 0;
    //     while offset < BLOCK_SIZE {
            
    //     }
    // }

    pub fn rmdir_from_data_block(&mut self,dir_name : &String){
        let mut offset = 0; 
        while offset < BLOCK_SIZE {
            let file_size : usize = bincode::deserialize(&self.data[4+offset..6+offset]).unwrap();
            let dir : DirectoryEntry = bincode::deserialize(&self.data[offset..offset+file_size]).unwrap();
            if dir.name == *dir_name {
                self.delete_some_block(offset, file_size);
                break;
            }
            offset += file_size;
        }
    }
    //todo这个部分删除数据块的部分还需要在考虑一下，直接覆盖时间复杂度太高
    pub fn delete_some_block(&mut self,offset : usize, block_count : usize){
        for i in offset..BLOCK_SIZE-block_count {
            self.data[i] = self.data[i + block_count]; 
        }
    }

    
}

//usr access tight


fn get_current_time() -> u32 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration.as_secs() as u32

}
