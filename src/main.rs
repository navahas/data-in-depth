// IEEE 754 binary implementation 

fn main() {
    let n: f32 = 12.12;
    let n_bits = n.to_bits();
    let exponent = n_bits >> 23;
    println!("{:032b}", exponent);
    let mask = 0xff;
    println!("{:032b}", mask);
    let exp_mask = exponent & mask;
    println!("{:032b}", exp_mask);
}
