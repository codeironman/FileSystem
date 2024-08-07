#![allow(dead_code)]
use crate::block::BlockGroup;
use fuser::Filesystem;
use std::time::Duration;
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
        let _ = _config.add_capabilities(fuser::consts::FUSE_ATOMIC_O_TRUNC);
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
        self.block_groups
            .write_file_offset(ino as usize, offset as usize, data);
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
        let data =
            self.block_groups
                .read_file_offset_size(ino as usize, _offset as usize, _size as usize);
        println!("{}", std::str::from_utf8(data.as_slice()).unwrap());
        reply.data(&data);
    }

    fn mkdir(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        reply: fuser::ReplyEntry,
    ) {
        println!(
            "mkdir called for parent={}, name={}",
            parent,
            name.to_string_lossy()
        );
        let name = name.to_string_lossy().into_owned();
        let attr = self.block_groups.bg_mkdir(name, parent as usize).unwrap();
        dbg!(attr);
        reply.entry(&Duration::from_secs(1), &attr, 0);
    }

    fn unlink(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        println!(
            "unlink called for parent={}, name={}",
            parent,
            name.to_string_lossy()
        );
        let name = name.to_string_lossy().into_owned();
        self.block_groups.bg_unlink(name, parent as usize);
        reply.ok()
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
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        println!("readdir called for ino={}", ino);
        self.block_groups
            .bg_list(ino as usize)
            .iter()
            .skip(offset as usize)
            .enumerate()
            .find(|(i, f)| {
                reply.add(
                    f.inode as u64,
                    *i as i64 + offset + 1,
                    f.get_type(),
                    &f.get_name(),
                )
            });

        reply.ok();
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        println!("getattr called for ino={}", ino);
        let attr = self.block_groups.bg_getattr(ino as usize);
        reply.attr(&Duration::new(0, 0), &attr);
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
        let attr = self
            .block_groups
            .bg_lookup(dir_name.to_string(), parent as usize);
        if let Some(file) = attr {
            reply.entry(&Duration::from_secs(1), &file, 0);
        } else {
            reply.error(libc::ENOENT)
        }
    }

    fn open(&mut self, _req: &fuser::Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        let _ = reply;
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
        println!(
            "create called for parent={}, name={}",
            _parent,
            _name.to_string_lossy()
        );
        let file_name = _name.to_string_lossy().into_owned();
        let attr = self.block_groups.bg_create(file_name, _parent as usize);
        if let Some(file) = attr {
            reply.created(&Duration::from_secs(0), &file, 0, 0, 0);
        }
    }
}

impl EXT2FS {
    pub fn new(name: String, pwd: String) -> Self {
        EXT2FS {
            //boot_block : boot,
            block_groups: BlockGroup::new_root(),
            path: "root".to_string(),
            user_name: name,
            password: pwd,
            current_inode_index: 0,
        }
    }
}
