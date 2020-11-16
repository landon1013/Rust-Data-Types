#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let a: u8 = 128; // u = unsigned, 8 bits, 0 - 255
    println!("a={}", a); // immutable

        // u = unsigned, 0 to 2^N-1
        // i = signed, -2^(N-1) .. 2^(N-1)-1
    let mut b: i8 = 0; // -128 -- 127
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);
}
