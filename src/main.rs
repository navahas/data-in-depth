// IEEE 754 binary implementation with edge cases
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    // Test normal numbers
    let n: f32 = 12.12;
    let (sign, exp, fract) = to_parts(n);
    let (sign_, exp_, mantissa) = decode(sign, exp, fract);
    let n_ = from_parts(sign_, exp_, mantissa);
    assert_eq!(n, n_);
    println!("Normal number: {} -> {}", n, n_);

    // Test edge cases
    test_edge_cases();
}

fn test_edge_cases() {
    let test_values = vec![
        0.0_f32,           // Positive zero
        -0.0_f32,          // Negative zero
        f32::INFINITY,     // Positive infinity
        f32::NEG_INFINITY, // Negative infinity
        f32::NAN,          // NaN
        1.0e-40_f32,       // Subnormal number
        f32::MIN_POSITIVE, // Smallest positive normal
        f32::MAX,          // Largest finite number
    ];

    for value in test_values {
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);

        println!("Original: {}, Reconstructed: {}", value, reconstructed);

        // Special handling for NaN comparison
        if value.is_nan() {
            assert!(reconstructed.is_nan());
        } else {
            // Handle signed zeros
            if value == 0.0 {
                assert_eq!(value.to_bits(), reconstructed.to_bits());
            } else {
                assert_eq!(value, reconstructed);
            }
        }
    }
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = (bits >> 31) & 1;
    let exp = (bits >> 23) & 0xff;
    // keeping only the lower 23 bits
    let fract = bits & 0x7fffff;
    (sign, exp, fract)
}

fn decode(sign: u32, exp: u32, fract: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    // Handle special cases based on exponent
    match exp {
        0 => {
            // Zero or subnormal numbers
            if fract == 0 {
                // Zero (positive or negative)
                (signed_1, 0.0, 0.0)
            } else {
                // Subnormal numbers
                let exponent = RADIX.powf((1 - BIAS) as f32); // Min exponent for subnormals
                let mantissa = compute_subnormal_mantissa(fract);
                (signed_1, exponent, mantissa)
            }
        }
        0xff => {
            // Infinity or NaN
            if fract == 0 {
                // Infinity
                (signed_1, f32::INFINITY, 1.0)
            } else {
                // NaN
                (1.0, f32::NAN, 1.0)
            }
        }
        _ => {
            // Normal numbers
            let exponent_ = (exp as i32) - BIAS;
            let exponent = RADIX.powf(exponent_ as f32);
            let mantissa = compute_normal_mantissa(fract);
            (signed_1, exponent, mantissa)
        }
    }
}

fn compute_normal_mantissa(fract: u32) -> f32 {
    let mut mantissa: f32 = 1.0; // Implicit leading 1 for normal numbers
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fract & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    mantissa
}

fn compute_subnormal_mantissa(fract: u32) -> f32 {
    let mut mantissa: f32 = 0.0; // No implicit leading 1 for subnormals
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fract & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    mantissa
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    // Handle special cases
    if exponent.is_nan() || mantissa.is_nan() {
        return f32::NAN;
    }

    if exponent.is_infinite() {
        if mantissa == 1.0 {
            return sign * f32::INFINITY;
        } else {
            return f32::NAN;
        }
    }

    if exponent == 0.0 && mantissa == 0.0 {
        // Handle signed zero
        if sign == -1.0 {
            return -0.0;
        } else {
            return 0.0;
        }
    }

    sign * exponent * mantissa
}
