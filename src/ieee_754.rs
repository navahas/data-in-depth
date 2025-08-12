// IEEE 754 binary implementation with edge cases
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

pub fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = (bits >> 31) & 1;
    let exp = (bits >> 23) & 0xff;
    // keeping only the lower 23 bits
    let fract = bits & 0x7fffff;
    (sign, exp, fract)
}

pub fn decode(sign: u32, exp: u32, fract: u32) -> (f32, f32, f32) {
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

pub fn compute_normal_mantissa(fract: u32) -> f32 {
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

pub fn compute_subnormal_mantissa(fract: u32) -> f32 {
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

pub fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
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

#[cfg(test)]
mod simple_precision {
    use super::*;

    #[test]
    fn test_normal_number() {
        let n: f32 = 12.12;
        let (sign, exp, fract) = to_parts(n);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(n, reconstructed);
    }

    #[test]
    fn test_positive_zero() {
        let value = 0.0_f32;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value.to_bits(), reconstructed.to_bits());
    }

    #[test]
    fn test_negative_zero() {
        let value = -0.0_f32;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value.to_bits(), reconstructed.to_bits());
    }

    #[test]
    fn test_positive_infinity() {
        let value = f32::INFINITY;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_negative_infinity() {
        let value = f32::NEG_INFINITY;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_nan() {
        let value = f32::NAN;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert!(reconstructed.is_nan());
    }

    #[test]
    fn test_subnormal_number() {
        let value = 1.0e-40_f32;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_min_positive_normal() {
        let value = f32::MIN_POSITIVE;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_max_finite() {
        let value = f32::MAX;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_negative_normal_number() {
        let value = -42.5_f32;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_one() {
        let value = 1.0_f32;
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }

    #[test]
    fn test_power_of_two() {
        let value = 8.0_f32; // 2^3
        let (sign, exp, fract) = to_parts(value);
        let (sign_, exp_, mantissa) = decode(sign, exp, fract);
        let reconstructed = from_parts(sign_, exp_, mantissa);
        assert_eq!(value, reconstructed);
    }
}
