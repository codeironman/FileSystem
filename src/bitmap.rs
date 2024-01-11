pub struct Bitmap {
    data: Vec<u8>,
}

impl Bitmap{
    pub fn new(size: usize)->Self{
        Bitmap{
            data: vec![0;size/8 + 1],
        }
    }
    pub fn get(self, index: usize) -> bool {
        self.data[index/8] & (1 << (7 - (index % 8))) != 0
    }
    pub fn set(&self, index: usize, value: bool) {
        if value {
            self.data[index/8] |= 1 << (7 - (index % 8));
        } else {
            self.data[index/8] &= !(1 << (7 - (index % 8)));
        }
    }
    pub fn free_idx(self) -> Option<usize>{
        for i in 0..(self.data.len()) {
            if self.data[i] != 0xff {
                for j in 0..8 {
                    if self.data[i] & (1 << (7 - j)) == 0 {
                        return Some(i * 8 + j);
                    }
                }
            }
        }
        return None;
    }
}