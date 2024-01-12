#[cfg(test)]
mod test{
    use crate::bitmap::Bitmap;
    use crate::file::*;
    #[test]
    fn test (){
        let DIR = DirectoryEntry::new("11".to_string(),FileType::Directory, 0, 2);
        let dir_data = bincode::serialize(&DIR).unwrap();
        //let a :DirectoryEntry= bincode::deserialize(&dir_data).unwrap();
        let b:u16 = bincode::deserialize(&dir_data[4..6]).unwrap();
        //let size  = u32::from_le_bytes(dir_data[4..6].try_into().unwrap());
        // let file_size = u16::from_le_bytes([data[4], data[5]]);

        assert_eq!(b,2);
    }

}
