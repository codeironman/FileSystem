use bincode::config::BigEndian;
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

    fn getxattr(
            &mut self,
            _req: &fuser::Request<'_>,
            _ino: u64,
            _name: &std::ffi::OsStr,
            _size: u32,
            reply: fuser::ReplyXattr,
        ) {
            
        
    }
    fn write(
            &mut self,
            _req: &fuser::Request<'_>,
            ino: u64,
            _fh: u64,
            _offset: i64,
            data: &[u8],
            _write_flags: u32,
            _flags: i32,
            _lock_owner: Option<u64>,
            reply: fuser::ReplyWrite,
        ) {
        self.block_groups[0].write_file(ino as usize, data)
        
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
                let block_group = BlockGroup::new();
                self.block_groups.push(block_group);
                self.block_groups.len() - 1
            },
        };
        //需要新建一个块
        self.block_groups[block_group_index].bg_mkdir(name, _parent as usize);


    }
    fn rmdir(&mut self, _req: &fuser::Request<'_>, _parent: u64, _name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        let dir_name = _name.to_string_lossy().into_owned();
        self.block_groups[0].bg_rmdir(_parent as usize, dir_name)
    }

    fn readdir(
            &mut self,
            _req: &fuser::Request<'_>,
            ino: u64,
            _fh: u64,
            _offset: i64,
            reply: fuser::ReplyDirectory,
        ) {
            self.block_groups[0].bg_list(ino as usize);
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
    pub fn ls(&self,block_group_index : usize) { 
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



