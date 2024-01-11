use std::mem;

fn main() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let size = data.len(); // 这将返回 5
    let mem = mem::size_of_val(&data);
    println!("Size of data slice: {}\n{}", size,mem);
}
