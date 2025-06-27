

pub fn print_types_info() {
    println!("i8:  Length 8-bit Signed:   min = {}, max = {}", i8::MIN, i8::MAX);
    println!("u8:  Length 8-bit Unsigned: min = {}, max = {}", u8::MIN, u8::MAX);
    println!();
    println!("i16:  Length 16-bit Signed:   min = {}, max = {}", i16::MIN, i16::MAX);
    println!("u16:  Length 16-bit Unsigned: min = {}, max = {}", u16::MIN, u16::MAX);
    println!();
    println!("i32:  Length 32-bit Signed:   min = {}, max = {}", i32::MIN, i32::MAX);
    println!("u32:  Length 32-bit Unsigned: min = {}, max = {}", u32::MIN, u32::MAX);
    println!();
    println!("i64:  Length 64-bit Signed:   min = {}, max = {}", i64::MIN, i64::MAX);
    println!("u64:  Length 64-bit Unsigned: min = {}, max = {}", u64::MIN, u64::MAX);
    println!();
    println!("i128: Length 128-bit Signed:   min = {}, max = {}", i128::MIN, i128::MAX);
    println!("u128: Length 128-bit Unsigned: min = {}, max = {}", u128::MIN, u128::MAX);
    println!();
    println!("isize: Length isize: min = {}, max = {}", isize::MIN, isize::MAX);
    println!("usize: Length usize: min = {}, max = {}", usize::MIN, usize::MAX);
    println!();
    println!("f32:  Length f32: min = {}, max = {}", f32::MIN, f32::MAX);
    println!("f64:  Length f64: min = {}, max = {}", f64::MIN, f64::MAX);
    println!();
}