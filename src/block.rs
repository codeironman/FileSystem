use crate::{bitmap::*, block, file::*};
use bincode;
use log::debug;
use serde::de::value::SeqDeserializer;
use std::{
    alloc::System,
    clone,
    cmp::{max, min},
    f32::consts::E,
    io::{Bytes, SeekFrom},
    mem,
    ops::Deref,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

pub const BLOCK_SIZE: usize = 1024; //每个数据块的大小为1kB
                                    //一个块的大小控制了，inode和bitmap的位图的大就是一个块的大小
                                    //那么块的数据也就确定了，1024*8
pub const BLOCK_COUNT: usize = BLOCK_SIZE * 8;

//part 1
pub struct BootBlock {}

//part 2
pub struct BlockGroup {
    superblock: SuperBlock,                     //超级块
    group_descriper_table: GroupDescriperTable, //块组描述
    inode_bitmap: Bitmap,                       // inode位图,判断inode是否分配
    block_bitmap: Bitmap,                       // 块位图// 用于标识区块组中所有区块的使用状态
    inode_table: Vec<Inode>,                    // inode表，用于描述数据块
    data_block: Vec<DataBlock>,
}
//todo，之后在管理超级块
pub struct SuperBlock {
    s_inodes_count: u32,
    s_block_count: u32,
    s_free_blocks_count: u32,
    s_free_inodes_count: u32,
    s_mtime: u32, //最后一次修改
    s_wtime: u32, //最后一次写入
}

pub struct GroupDescriperTable {
    bg_block_bitmap: u32,
    bg_inode_bitmap: u32,
    bg_inode_table: u32,
    bg_free_block_count: u16,
    bg_free_inode_count: u16,
    bg_used_dirs_count: u16,
}

#[derive(Debug)]
pub struct Inode {
    i_mode: u16, //16位置用于表示文件的类型和权限
    i_uid: u16,  //用户id
    i_size: u32,
    i_atime: u32, //存储的是秒数
    i_ctime: u32,
    i_mtime: u32,
    i_dtime: u32,
    i_gid: u16, //组id
    i_links_count: u16,
    i_block: [Option<u32>; 15], //指向的是数据块
    //file_type: FileType,可有可无
    pub direct_pointer: [Option<u32>; 12], //直接索引，前12个块 Some是指向数字的指针，要么就是None
    pub singly_indirect_block: Option<u32>, //一级索引，第13个块
    pub doubly_indirect_block: Option<u32>, //二级索引，第14个块
    pub triply_indirect_block: Option<u32>, //三级索引，第15个块
}
#[derive(Debug)]
struct DataBlock {
    pub data: [u8; BLOCK_SIZE as usize], //每个文件块的大小为1kB
}

impl BlockGroup {
    pub fn new_root() -> Self {
        let mut bg = BlockGroup::new();

        bg.inode_bitmap.set(0, true);
        let inode = bg.inode_table.get_mut(0).unwrap();
        // 777 dir
        inode.i_mode = 0x41ff;
        bg.add_entry_to_directory(".".to_string(), 1);
        bg.add_entry_to_directory("..".to_string(), 1);
        bg
    }

    pub fn new() -> Self {
        BlockGroup {
            superblock: SuperBlock::new(),
            group_descriper_table: GroupDescriperTable::new(),
            inode_bitmap: Bitmap::new(BLOCK_SIZE),
            block_bitmap: Bitmap::new(BLOCK_SIZE),
            inode_table: vec![Inode::new(); BLOCK_SIZE * 8],
            data_block: vec![DataBlock::new(); BLOCK_SIZE * 8],
        }
    }

    pub fn full(&self) -> bool {
        self.block_bitmap.free_index() == None
    }

    pub fn read_file_offset_size(
        &mut self,
        inode_index: usize,
        offset: usize,
        mut size: usize,
    ) -> Vec<u8> {
        let inode = self.inode_table.get(inode_index - 1).unwrap();
        size = min(offset + size, inode.i_size as usize) - offset;
        let mut data: Vec<u8> = vec![0; size];
        let (mut read_entry_block_idx, mut inner_offset) = convert_offset(offset);

        let mut read_pos = 0;
        loop {
            let block = self.get_block_by_inode_rel_block(inode_index, read_entry_block_idx);
            let read_count = min(BLOCK_SIZE - inner_offset, size - read_pos);

            data[read_pos..read_pos + read_count]
                .copy_from_slice(&block.read()[inner_offset..inner_offset + read_count]);
            read_pos += read_count;
            read_entry_block_idx += 1;
            inner_offset = 0;
            if read_pos >= size {
                break;
            }
        }

        data
    }

    pub fn write_file_offset(&mut self, inode_idx: usize, offset: usize, data: &[u8]) {
        let inode = self.inode_table.get_mut(inode_idx - 1).unwrap();
        inode.inode_update_size(max(inode.i_size, (offset + data.len()) as u32));
        let (mut write_entry_block_idx, mut inner_offset) = convert_offset(offset);

        let mut write_pos = 0;
        loop {
            let block = self.get_block_by_inode_rel_block(inode_idx, write_entry_block_idx);
            let write_count = min(BLOCK_SIZE - inner_offset, data.len() - write_pos);

            block.write(&data[write_pos..write_pos + write_count], inner_offset);
            write_pos += write_count;
            write_entry_block_idx += 1;
            inner_offset = 0;
            if write_pos >= data.len() {
                break;
            }
        }
    }

    fn get_block_by_inode_rel_block(&mut self, inode: usize, block_idx: usize) -> &mut DataBlock {
        // todo: 多级混合索引
        match block_idx {
            0..=11 => {
                match self.inode_table[inode - 1].direct_pointer[block_idx] {
                    Some(index) => self.data_block.get_mut(index as usize).unwrap(),
                    None => {
                        // 自动扩容
                        let datablock = self.get_block_for_file();
                        self.inode_table[inode - 1].direct_pointer[block_idx] =
                            Some(datablock as u32);
                        &mut self.data_block[datablock]
                    }
                }
            }
            _ => panic!("not implemented"),
        }
    }

    pub fn bg_list(&self, parent_inode: usize) -> Vec<DirectoryEntry> {
        let mut all_dirs: Vec<DirectoryEntry> = vec![];
        for index in self.inode_table[parent_inode as usize - 1].direct_pointer {
            if let Some(i_block) = index {
                all_dirs.append(&mut self.data_block[i_block as usize].get_all_dirs_name());
            }
        }
        all_dirs
    }

    pub fn bg_rmdir(&mut self, parent_inode: usize, name: String) {
        for block_index in self.inode_table[parent_inode - 1].direct_pointer {
            if let Some(index) = block_index {
                self.data_block[index as usize].rmdir_from_data_block(&name);
            }
        }
    }

    pub fn bg_mkdir(&mut self, name: String, parent_inode: usize) -> Option<fuser::FileAttr> {
        let child_inode = self.add_entry_to_directory(name, parent_inode,FileType::Directory);
        self.inode_table[child_inode - 1].init_as_dir();
        self.add_entry_to_directory(".".to_string(), child_inode,FileType::Directory);
        self.add_entry_to_directory("..".to_string(), child_inode,FileType::Directory);
        Some(self.inode_table[parent_inode - 1].get_file_attr(child_inode as u64))
    }

    pub fn bg_lookup(&mut self, name: String, parent_inode: usize) -> Option<fuser::FileAttr> {
        for block_index in self
            .inode_table
            .get(parent_inode - 1)
            .unwrap()
            .direct_pointer
        {
            if let Some(index) = block_index {
                let data_block = &self.data_block[index as usize];
                let dirs = data_block.get_all_dirs_name();
                for dir in dirs {
                    if dir.name == name {
                        let inode = &self.inode_table[index as usize - 1];
                        return Some(inode.get_file_attr(index as u64));
                    }
                }
            }
        }
        None
    }

    pub fn bg_getattr(&self, inode_index: usize) -> fuser::FileAttr {
        let inode = self.inode_table.get(inode_index - 1).unwrap();
        inode.get_file_attr(inode_index as u64)
    }

    pub fn bg_create(&mut self, name: String, parent_inode: usize) -> Option<fuser::FileAttr> {
        let child_inode = self.add_entry_to_directory(name, parent_inode,FileType::Regular);
        self.inode_table[child_inode - 1].init_as_file();
        Some(self.bg_getattr(child_inode))
    }

    pub fn add_entry_to_directory(&mut self, name: String, parent_inode: usize, filetype : FileType) -> usize {
        let inode_index = self.get_inode(); //分配一个inode
        let mut dir = DirectoryEntry::new(name, filetype, inode_index as u32, 0);
        let (dir_data, dir_size) = dir.to_bytes();//修改了目录的大小
        for &block_index in self
            .inode_table
            .get(parent_inode - 1)
            .unwrap()
            .direct_pointer
            .iter()
            .take_while(|x: &&Option<u32>| x.is_some())
        {
            if let Some(index) = block_index {
                let data_block = &mut self.data_block[index as usize];
                if data_block.count_free_bytes() >= dir_size as u16 {
                    data_block.write(
                        &dir_data,
                        BLOCK_SIZE - data_block.count_free_bytes() as usize,
                    );
                    return inode_index;
                }
            }
        }

        let new_block_index = self.get_block_for_file();
        let data_block = &mut self.data_block[new_block_index];
        data_block.write(&dir_data, 0);
        *self.inode_table[parent_inode - 1]
            .direct_pointer
            .iter_mut()
            .find(|x| x.is_none())
            .unwrap() = Some(new_block_index as u32);

        inode_index
    }

    fn get_inode(&mut self) -> usize {
        match self.inode_bitmap.free_index() {
            Some(index) => {
                self.inode_bitmap.set(index, true);
                return index + 1
            }
            None => {
                panic!("no free inode")
            }
        }
    }
    //找到空的块
    pub fn get_block_for_file(&mut self) -> usize {
        match self.block_bitmap.free_index() {
            Some(index) => {
                self.block_bitmap.set(index, true);
                return index + 1
            }
            None => panic!("no free block"),
        }
    }
}

impl SuperBlock {
    pub fn new() -> Self {
        SuperBlock {
            s_inodes_count: BLOCK_COUNT as u32,
            s_block_count: BLOCK_COUNT as u32,
            s_free_blocks_count: BLOCK_COUNT as u32,
            s_free_inodes_count: BLOCK_COUNT as u32,
            s_mtime: get_current_time(),
            s_wtime: get_current_time(),
        }
    }
}

impl GroupDescriperTable {
    pub fn new() -> Self {
        GroupDescriperTable {
            bg_block_bitmap: 2,
            bg_inode_bitmap: 3,
            bg_inode_table: 4,
            bg_free_block_count: BLOCK_COUNT as u16,
            bg_free_inode_count: BLOCK_COUNT as u16,
            bg_used_dirs_count: 0,
        }
    }
    pub fn gdt_update(&mut self, block_change: i16, inode_change: i16, dir_change: i16) {
        self.bg_free_block_count = (self.bg_free_block_count as i16 + block_change) as u16;
        self.bg_free_inode_count = (self.bg_free_inode_count as i16 + inode_change) as u16;
        self.bg_used_dirs_count = (self.bg_used_dirs_count as i16 + dir_change) as u16;
    }
    pub fn full(&self) -> bool {
        self.bg_free_block_count == 0
    }
}
enum Permission {
    Read = 0x0100,
    Write = 0x0080,
    Excute = 0x0040, //可执行
}
//枚举不支持｜和&，先转化为u16
pub enum Filetype {
    Unknow = 0x0000,
    Regular = 0x8000,
    Directory = 0x4000,
    // CharacterDevice = 0x2000,
    // BlockDevice = 0x6000,
    // FIFO = 0x1000,,
    // Socket = 0xC000,
    // SymbolicLink = 0xA000,
}

impl Clone for Inode {
    fn clone(&self) -> Self {
        Inode {
            i_mode: self.i_mode,
            i_size: self.i_size,
            i_atime: self.i_atime,
            i_ctime: self.i_ctime,
            i_mtime: self.i_mtime,
            i_dtime: self.i_dtime,
            i_block: self.i_block,
            direct_pointer: self.direct_pointer.clone(),
            singly_indirect_block: self.singly_indirect_block.clone(),
            doubly_indirect_block: self.doubly_indirect_block.clone(),
            triply_indirect_block: self.triply_indirect_block.clone(),
            i_uid: self.i_uid,
            i_gid: self.i_gid,
            i_links_count: self.i_links_count,
        }
    }
}

impl Inode {
    fn new() -> Self {
        Inode {
            i_mode: 0x0000,
            i_size: 0,
            i_atime: get_current_time(),
            i_ctime: get_current_time(),
            i_mtime: get_current_time(),
            i_dtime: get_current_time(),
            i_block: Default::default(), //初始化为15个None的数组
            direct_pointer: [None; 12],
            singly_indirect_block: None,
            doubly_indirect_block: None,
            triply_indirect_block: None,
            i_uid: 0,
            i_gid: 0,
            i_links_count: 1,
        }
    }

    pub fn get_file_attr(&self, inode_index: u64) -> fuser::FileAttr {
        fuser::FileAttr {
            ino: inode_index as u64,
            size: self.i_size as u64,
            // todo
            blocks: 0,
            atime: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.i_atime as u64),
            mtime: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.i_mtime as u64),
            ctime: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.i_ctime as u64),
            crtime: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.i_ctime as u64),
            kind: match self.i_mode & 0xf000 {
                0x8000 => fuser::FileType::RegularFile,
                0x4000 => fuser::FileType::Directory,
                _ => fuser::FileType::RegularFile,
            },
            perm: self.i_mode & 0x0fff,
            nlink: self.i_links_count as u32,
            uid: self.i_uid as u32,
            gid: self.i_gid as u32,
            // 不熟这些 attr 先不实现
            rdev: 0,
            flags: 0,
            blksize: BLOCK_SIZE as u32,
            padding: 0,
        }
    }

    pub fn inode_update_size(&mut self, size: u32) {
        self.i_size += size + self.i_size;
        self.i_ctime = get_current_time();
    }

    pub fn init_as_file(&mut self,) {
        self.i_mode = 0x81ff;
        self.i_size = 0;
        self.i_atime = get_current_time();
        self.i_ctime = get_current_time();
        self.i_mtime = get_current_time();
        self.direct_pointer = [None; 12];
    }

    pub fn init_as_dir(&mut self) {
        self.i_mode = 0x41ff;
        self.i_size = 0;
        self.i_atime = get_current_time();
        self.i_ctime = get_current_time();
        self.i_mtime = get_current_time();
        self.direct_pointer = [None; 12];
    }
}

impl Clone for DataBlock {
    fn clone(&self) -> Self {
        DataBlock {
            data: self.data.clone(),
        }
    }
}

impl DataBlock {
    pub fn get_file(&self) -> DirectoryEntry {
        bincode::deserialize(&self.data).unwrap()
    }
    pub fn new() -> Self {
        DataBlock {
            data: [0; BLOCK_SIZE as usize],
        }
    }
    pub fn write(&mut self, data: &[u8], offset: usize) {
        for (i, &byte) in data.iter().enumerate().take(BLOCK_SIZE) {
            self.data[i + offset] = byte;
        }
    }

    pub fn read(&self) -> &[u8] {
        &self.data
    }
    pub fn count_free_bytes(&self) -> u16 {
        let mut offset: usize = 0;
        while offset + 8 < BLOCK_SIZE {
            let file_size: u16 =
                self.data[offset + 4] as u16 + ((self.data[offset + 5] as u16) << 8);
            if file_size == 0 {
                break;
            }
            offset += file_size as usize;
        }
        return (BLOCK_SIZE - offset) as u16;
    }
    pub fn get_all_dirs_name(&self) -> Vec<DirectoryEntry> {
        let mut offset = 0;
        let mut dir_vec: Vec<DirectoryEntry> = vec![];
        while offset + 8 < BLOCK_SIZE {
            let file_size = self.data[offset + 4] as u16 + ((self.data[offset + 5] as u16) << 8);
            //dbg!(file_size);
            //bincode::deserialize(!(&self.data[4 + offset..6 + offset])).unwrap(); //从第四个字节开始解析2个字节为文件的大小
            if file_size == 0 {
                break;
            }
            let dir: DirectoryEntry =
                bincode::deserialize(&self.data[offset..offset + file_size as usize]).unwrap();
            dir_vec.push(dir);
            offset += file_size as usize;
        }
        dir_vec
    }

    pub fn rmdir_from_data_block(&mut self, dir_name: &String) {
        let mut offset = 0;
        while offset < BLOCK_SIZE {
            let file_size: usize =
                bincode::deserialize(&self.data[4 + offset..6 + offset]).unwrap();
            let dir: DirectoryEntry =
                bincode::deserialize(&self.data[offset..offset + file_size]).unwrap();
            if dir.name == *dir_name {
                self.delete_some_block(offset, file_size);
                break;
            }
            offset += file_size;
        }
    }
    //todo这个部分删除数据块的部分还需要在考虑一下，直接覆盖时间复杂度太高
    pub fn delete_some_block(&mut self, offset: usize, block_count: usize) {
        for i in offset..BLOCK_SIZE - block_count {
            self.data[i] = self.data[i + block_count];
        }
    }
}

//usr access tight

pub fn get_current_time() -> u32 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_secs() as u32
}

pub fn convert_offset(offset: usize) -> (usize, usize) {
    (offset / BLOCK_SIZE, offset % BLOCK_SIZE)
}
