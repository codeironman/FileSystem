mod disk;
mod file;
use disk::EXT2FS;
mod bitmap;
mod block;
use fuser::MountOption;
use log::info;

fn main() {
    let mountpoint = "/Users/caofengyi/code/os/mount";
    let name = "123".to_string();
    let pw = "abc".to_string();
    let filesystem = EXT2FS::new(name, pw);
    info!("Begin to load our filesystem");
    fuser::mount2(
        filesystem,
        mountpoint,
        &[MountOption::FSName("myfs".to_string())],
    )
    .unwrap();
}
