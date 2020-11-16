#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let a: u8 = 128; // u = unsigned, 8 bits, 0 - 255
    println!("a={}", a); // immutable

        // u = unsigned, 0 to 2^N-1
        // i = signed, -2^(N-1) .. 2^(N-1)-1
    let mut b: i8 = 0; // -128 -- 127
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789; // i32 = 32 bits = 4 bytes
    println! ("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}=bit OS",
        z, size_of_z, size_of_z*8);
}
