fn main() {
    let value_i16: u16 = 100; // u16 类型的值
    let value_i32: i32 = -50; // i32 类型的值

    // 将 u16 值转换为 i32，然后执行加法
    let sum = value_i16 as i32 + value_i32;

    // 将 u16 值转换为 i32，然后执行减法
    let difference = value_i16 as i32 - value_i32;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
