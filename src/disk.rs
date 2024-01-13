use bincode::config::BigEndian;
use std::collections::btree_map::Entry;
use std::collections::btree_set::Union;
use std::{usize, result};

use bincode::Error;
use fuser::Filesystem;
use crate::file::*;
use crate::block::{BlockGroup,BootBlock,SuperBlock, self};
pub struct EXT2FS{
    //boot_block : Boot_Block,
    block_groups : Vec<BlockGroup>,
    path : String,
    current_inode_index : usize,
    user_name : String,
    password : String
}

impl Filesystem for EXT2FS {
    fn init( &mut self, _req: &fuser::Request<'_>, _config: &mut fuser::KernelConfig) -> Result<(), std::ffi::c_int> {
        Ok(())
    }

    fn create(
            &mut self,
            _req: &fuser::Request<'_>,
            _parent: u64,
            _name: &std::ffi::OsStr,
            _mode: u32,
            _umask: u32,
            _flags: i32,
            reply: fuser::ReplyCreate,
        ) {
        
    }

    fn lookup(&mut self, _req: &fuser::Request<'_>, _parent: u64, _name: &std::ffi::OsStr, reply: fuser::ReplyEntry) {
        
    }

    fn readdir(
            &mut self,
            _req: &fuser::Request<'_>,
            _ino: u64,
            _fh: u64,
            _offset: i64,
            mut reply: fuser::ReplyDirectory,
        ) {

        for (i,entry) in self.block_groups[0].bg_list(_ino).into_iter().enumerate().skip(_offset as usize){
            if reply.add(entry.inode as u64, (i+1) as i64, entry.get_type(), entry.get_name()){
                break;
            }
        }
        reply.ok();
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, _ino: u64, reply: fuser::ReplyAttr) {
        self.block_groups[0].bg_getattr(_ino);

    }

    fn write(
            &mut self,
            _req: &fuser::Request<'_>,
            _ino: u64,
            _fh: u64,
            _offset: i64,
            _data: &[u8],
            _write_flags: u32,
            _flags: i32,
            _lock_owner: Option<u64>,
            reply: fuser::ReplyWrite,
        ) {
        
        
    }




    fn read(
            &mut self,
            _req: &fuser::Request<'_>,
            _ino: u64,
            _fh: u64,
            _offset: i64,
            _size: u32,
            _flags: i32,
            _lock_owner: Option<u64>,
            reply: fuser::ReplyData,
        ) {
        
    }

    fn mkdir(
            &mut self,
            _req: &fuser::Request<'_>,
            _parent: u64,
            name: &std::ffi::OsStr,
            _mode: u32,
            _umask: u32,
            reply: fuser::ReplyEntry,
        ) {
        let name = name.to_string_lossy().into_owned();
        let block_group_index = match self.get_block_group() {
            Some(index) => index,
            None => {
                //需要新建一个块
                let block_group = BlockGroup::new();
                self.block_groups.push(block_group);
                self.block_groups.len() - 1
            },
        };
        self.block_groups[block_group_index].bg_mkdir(name, _parent as usize);
    }

    fn rmdir(&mut self, _req: &fuser::Request<'_>, _parent: u64, _name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        let dir_name = _name.to_string_lossy().into_owned();
        self.block_groups[0].bg_rmdir(_parent as usize, dir_name)
    }

    fn open(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        
    }

    
}

impl EXT2FS {
    pub fn new(name :String, pwd : String)-> Self {
        //新建一个大块
        let root_block = BlockGroup::new();
        //将root文件夹放入第一个大块中
        EXT2FS{
            //boot_block : boot,
            block_groups : vec![root_block],
            path : "root".to_string(),
            user_name : name,
            password : pwd,
            current_inode_index : 0,
        }

    }

    pub fn get_block_group(&self) -> Option<usize>{
        self.block_groups
            .iter()
            .position(|x| !x.full())
    }

    pub fn cd() {

    }
    pub fn create() {

    }
    pub fn close() {

    }
    pub fn exitsys() {

    }

}

pub fn cal_group_block(inode_index: usize) -> (usize, usize) {
    let inode_block_index = inode_index / block::BLOCK_COUNT;
    let inode_offset = inode_index % block::BLOCK_COUNT;
    (inode_block_index, inode_offset)
}



