#[derive(Clone, Copy)]
pub struct Q7(pub i8);

// values between -1..1
impl From<f64> for Q7 {
    fn from(number: f64) -> Self {
        if number >= 1.0 {
            Q7(127)
        } else if number <= -1.0 {
            Q7(-128)
        } else {
            Q7((number * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_to_q7_boundary_values() {
        assert_eq!(Q7::from(1.0).0, 127);
        assert_eq!(Q7::from(-1.0).0, -128);
        assert_eq!(Q7::from(1.5).0, 127);
        assert_eq!(Q7::from(-1.5).0, -128);
    }

    #[test]
    fn test_f64_to_q7_zero() {
        assert_eq!(Q7::from(0.0).0, 0);
    }

    #[test]
    fn test_f64_to_q7_small_values() {
        assert_eq!(Q7::from(0.5).0, 64);
        assert_eq!(Q7::from(-0.5).0, -64);
        assert_eq!(Q7::from(0.25).0, 32);
        assert_eq!(Q7::from(-0.25).0, -32);
    }

    #[test]
    fn test_q7_to_f64_conversion() {
        let q = Q7(64);
        let f: f64 = q.into();
        assert!((f - 0.5).abs() < 0.01);

        let q = Q7(-64);
        let f: f64 = q.into();
        assert!((f + 0.5).abs() < 0.01);
    }

    #[test]
    fn test_f32_conversions() {
        let q = Q7::from(0.5f32);
        assert_eq!(q.0, 64);

        let f: f32 = q.into();
        assert!((f - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_round_trip_conversion() {
        let original = 0.75f64;
        let q = Q7::from(original);
        let converted: f64 = q.into();
        assert!((original - converted).abs() < 0.01);
    }
}
