#[cfg(test)]
mod test{
    use crate::bitmap::Bitmap;

    #[test]
    fn test_bitmap(){
        let mut bitmap = Bitmap::new(100);
        bitmap.set(99,true);
        assert_eq!(bitmap.get(99), true);
        bitmap.set(99,false);
        assert_eq!(bitmap.get(99), false);
    }
}
