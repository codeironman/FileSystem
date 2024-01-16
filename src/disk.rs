use std::f32::consts::E;
use std::time::Duration;

use crate::block::{self, BlockGroup, BootBlock, SuperBlock};
use crate::file::*;
use bincode::config::BigEndian;
use fuser::Filesystem;
use log::debug;
use log::{info, logger};
pub struct EXT2FS {
    //boot_block : Boot_Block,
    block_groups: BlockGroup,
    path: String,
    current_inode_index: usize,
    user_name: String,
    password: String,
}

impl Filesystem for EXT2FS {
    fn init(
        &mut self,
        _req: &fuser::Request<'_>,
        _config: &mut fuser::KernelConfig,
    ) -> Result<(), std::ffi::c_int> {
        println!("init called");
        self.block_groups = BlockGroup::new_root();
        Ok(())
    }
    fn write(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    ) {
        println!("write called for ino={}", ino);
        self.block_groups.write_file_offset(ino as usize, offset as usize, data);
        reply.written(data.len() as u32);
    }

    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        println!("read called for ino={}", ino);
        let data = self.block_groups.read_file(ino as usize);
        println!("{}", std::str::from_utf8(data.as_slice()).unwrap());
        reply.data(&data);
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
        println!(
            "mkdir called for parent={}, name={}",
            _parent,
            name.to_string_lossy()
        );
        let name = name.to_string_lossy().into_owned();
        //需要新建一个块
        self.block_groups.bg_mkdir(name, _parent as usize);
    }
    fn rmdir(
        &mut self,
        _req: &fuser::Request<'_>,
        _parent: u64,
        _name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        println!(
            "rmdir called for parent={}, name={}",
            _parent,
            _name.to_string_lossy()
        );
        let dir_name = _name.to_string_lossy().into_owned();
        self.block_groups.bg_rmdir(_parent as usize, dir_name);
        reply.ok();
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        _offset: i64,
        reply: fuser::ReplyDirectory,
    ) {
        println!("readdir called for ino={}", ino);
        self.block_groups.bg_list(ino as usize);
        reply.ok();
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, _ino: u64, reply: fuser::ReplyAttr) {
        println!("getattr called for ino={}", _ino);
        let inode = self.block_groups.bg_getattr(_ino as usize);
    }

    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        println!(
            "lookup called for parent={}, name={}",
            parent,
            name.to_string_lossy()
        );
        let dir_name = name.to_string_lossy().into_owned();
        let attr = self.block_groups.bg_lookup(dir_name, parent as usize );
        if let Some(file) = attr {
            reply.entry(&Duration::from_secs(1), &file, 0);
        }

    }



    fn open(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
    
    }
}

impl EXT2FS {
    pub fn new(name: String, pwd: String) -> Self {
        //新建一个大块
        let root_block = BlockGroup::new();
        //将root文件夹放入第一个大块中
        EXT2FS {
            //boot_block : boot,
            block_groups: root_block,
            path: "root".to_string(),
            user_name: name,
            password: pwd,
            current_inode_index: 0,
        }
    }
    // pub fn get_block_group(&self) -> Option<usize> {
    //     self.block_groups.iter().position(|x| !x.full())
    // }

    pub fn cd() {}
    pub fn create() {}
    pub fn close() {}
    pub fn exitsys() {}
}
