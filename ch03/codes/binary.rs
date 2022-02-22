const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("a: {a:016b} {a}");
    println!("b: {b:016b} {b}");

    let a: f32 = 42.42;
    #[allow(clippy::transmute_float_to_int)]
    let frankentype: u32 = unsafe { std::mem::transmute(a) };
    println!("{frankentype}");
    println!("{frankentype:032b}");

    #[allow(clippy::transmute_int_to_float)]
    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{b}");
    assert_eq!(a, b);

    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{a} vs {b}");
    println!("{a:032b} vs {b:032b}");
    if b == -1430532899 {
        println!("Your machine use little endian")
    } else if b == -573785174 {
        println!("Your machine use big endian")
    } else {
        println!("Your machine use odd endian")
    }

    let n: f32 = 42.42;
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{n} -> {n_}");
    println!("field    | bits                            | value");
    println!("sign     | {sign:01b}                               | {sign_}");
    println!("exponent | {exp:08b}                        | {exp_}");
    println!("mantissa | {frac:023b} | {mant}");

    println!("max: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits: u32 = n.to_bits();

    let sign = bits >> 31;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1f32).powf(sign as f32);
    let exponent = RADIX.powf((exponent as i32 - BIAS) as f32);
    let mantissa = {
        let mut mantissa: f32 = 1.0;
        for i in 0..23 {
            let mask = 1 << i;
            let one_at_bit_i = fraction & mask;
            if one_at_bit_i != 0 {
                let weight = 2f32.powf(i as f32 - 23.0);
                mantissa += weight
            }
        }
        mantissa
    };
    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_mock_rand() {
        for n in u8::MIN..=u8::MAX {
            let n = mock_rand(n);
            assert!(0.0 <= n);
            assert!(n <= 1.0);
        }
    }
}
