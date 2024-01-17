#[cfg(test)]
mod test {
    use std::{os::unix::process, mem::discriminant};

    #[test]
    fn test() {
        use crate::file::*;
        let DIR = DirectoryEntry::new("11".to_string(), FileType::Directory, 0, 2);
        let dir_data = bincode::serialize(&DIR).unwrap();
        //let a :DirectoryEntry= bincode::deserialize(&dir_data).unwrap();
        let b: u16 = bincode::deserialize(&dir_data[4..6]).unwrap();
        //let size  = u32::from_le_bytes(dir_data[4..6].try_into().unwrap());
        // let file_size = u16::from_le_bytes([data[4], data[5]]);

        assert_eq!(b, 2);
    }

    #[test]
    fn test_bitmap() {
        use crate::bitmap::Bitmap;
        let mut bm = Bitmap::new(128);
        assert_eq!(bm.get(1), false);
        bm.set(1, true);
        assert_eq!(bm.get(1), true);
    }


    #[test]
    fn test_bincode() {
        use crate::file::*;
        use bincode::deserialize;
    
        let mut dir = DirectoryEntry::new("11".to_string(), FileType::Directory, 0, 100);
        let (dir_data, dir_size) = dir.to_bytes();
        // 打印 dir_data 的十六进制表示
        println!("dir_data: {:?},{},{}", dir_data,dir_data.len(),dir_size);
        let c = dir_data[4];
        println!("c = {}",c);
        let t : usize= bincode::deserialize(&dir_data[4..6]).unwrap();
        println!("t = {}",t);
        // 或者更易于阅读的格式
        println!("dir_data (hex): {:?}", dir_data.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join(" "));
    
        assert_eq!(200, t);
    }
    

}
