mod ieee_754;
use ieee_754::*;

fn main() {
    let n: f32 = 12.12;
    let (sign, exp, fract) = to_parts(n);
    let (sign_, exp_, mantissa) = decode(sign, exp, fract);
    let n_ = from_parts(sign_, exp_, mantissa);
    assert_eq!(n, n_);
}
