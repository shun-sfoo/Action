mod mock_rand;
mod visualizing_f32;

fn main() {
    f1();
    f2();
    f3();
    f4();
    f5();
    visualizing_f32::calculate();
    mock_rand::mock();
}

/// {:016b} 占用16位，位数不足则在其左侧补0
fn f1() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("a : {:016b}", a);
    println!("b : {:016b}", b);
}

fn f2() {
    let a: f32 = 42.42;
    // std::mem::transmute 要求在不影响任何底层的位数据的情况下，把一个f32直接解释成u32
    let franketype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", franketype);
    println!("{:032b}", franketype);

    let b: f32 = unsafe { std::mem::transmute(franketype) };

    println!("{}", b);
    assert_eq!(a, b);
}

fn f3() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];

    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };

    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{} vs {}", a, b);
}

fn f4() {
    let mantissa: u32 = 0b010_1001_1010_1110_0001_0100; // 273154
    println!("{}", mantissa);
}

fn f5() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    println!("{}", mantissa);
}
