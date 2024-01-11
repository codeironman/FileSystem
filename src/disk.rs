use fuser::Filesystem;
use crate::file::*;
use crate::block::{Block_Group,Boot_Block,SuperBlock, self};
pub struct EXT2FS{
    //boot_block : Boot_Block,
    blocks : Vec<Block_Group>,
    current_directory : String,
    current_inode_index : u32,
    user_name : String,
    password : String
}




impl Filesystem for EXT2FS {
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
            _name: &std::ffi::OsStr,
            _mode: u32,
            _umask: u32,
            reply: fuser::ReplyEntry,
        ) {
        let name = _name.to_string_lossy().into_owned();
        let block_index = self.get_block_group();
        //存在空闲的块
        if block_index == -1 {
            let block_group = Block_Group::new();
            self.blocks.push(block_group);
            let block_index = self.blocks.len() - 1;
        }
        //需要新建一个块
        let index = self.blocks[block_index as usize].get_inode_index(_parent as usize);
        let entry = DirectoryEntry::new(name,FileType::Directory);
        self.blocks[block_index as usize].write_to_block(entry); 

    }
    fn rmdir(&mut self, _req: &fuser::Request<'_>, _parent: u64, _name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        
    }

    fn open(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        
    }

    
}

impl EXT2FS {
    pub fn new(name :String, pwd : String)-> Self {
        //新建一个大块
        let root_block = Block_Group::new();
        //将root文件夹放入第一个大块中
        EXT2FS{
            //boot_block : boot,
            blocks : vec![root_block],
            current_directory : "root".to_string(),
            user_name : name,
            password : pwd,
            current_inode_index : 0,
        }

    }
    pub fn ls(&self) { 
        self.blocks[0].list()
    }
    pub fn  get_block_group(&self) -> i32{
        let mut block_index = 0;
        for block_group in &self.blocks {
            if !block_group.group_descriper_table.full() {
                return block_index;
            }
            else {
                block_index += 1;
            }
        }
        -1
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



