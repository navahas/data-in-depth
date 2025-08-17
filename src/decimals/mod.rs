mod ieee_754;
mod q_format;

use ieee_754::*;
use q_format::*;

pub fn ieee() {
    let n: f32 = 12.12;
    let (sign, exp, fract) = to_parts(n);
    let (sign_, exp_, mantissa) = decode(sign, exp, fract);
    let n_ = from_parts(sign_, exp_, mantissa);
    assert_eq!(n, n_);
}

pub fn q_format() {
    let n: f64 = 0.241;

    let q = Q7::from(n);
    let back: f64 = f64::from(q);

    // size of 1 LSB in Q7
    let lsb = 1.0 / 128.0;
    //
    // Because Q7 saturates: [-1.0, 127/128]
    let expected = n.clamp(-1.0, 127.0 / 128.0);

    // For truncation-based impl:
    assert!((back - expected).abs() < lsb);

    // If rounding, this tighter bound is valid:
    // assert!((back - expected).abs() <= lsb / 2.0);

    println!("f64 bits: {:064b}", n.to_bits());
    println!("Q7  bits: {:08b} (raw)", q.0 as u8);
    println!("n: {n}, q: {}, back: {}", q.0, back);
}

// between 0..1
fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000; //0.5f32
    let large_n = (n as u32) << 15; // n to mantissa 8 + 15 = 23
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

pub fn rand_f32() -> f32 {
    mock_rand(12)
}
