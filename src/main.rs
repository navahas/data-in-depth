// IEEE 754 binary implementation 

fn main() {
    let n: f32 = 12.12;
    let n_bits = n.to_bits();
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        //println!("\n");
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        //if one_at_bit_i != 0 {
        // if (n_bits  & (1 << i)) != 0 {
        if ((n_bits >> i) & 1) == 1 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
            println!("{:>2} - {:032b} - {:032b}", i, mantissa.to_bits(), weight.to_bits());
        }
    }

    //println!("{}", mantissa);
}
