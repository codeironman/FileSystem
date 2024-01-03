use fuser::Filesystem;

use crate::block::{Block_Group,Boot_Block};
pub struct EXT2FS{
    //boot_block : Boot_Block,
    block : Vec<Block_Group>,
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
        
    }
    fn rmdir(&mut self, _req: &fuser::Request<'_>, _parent: u64, _name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        
    }

    fn open(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        
    }

    
}

impl EXT2FS {
   pub fn new(name :String, pw : String)-> Self {
        EXT2FS{
            //boot_block : boot,
            block : vec![],
            user_name : name,
            password : pw
        }
    }

    pub fn ls() {

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
