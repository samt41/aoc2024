use core::simd::prelude::*;

pub fn part1(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();
    let mut sum: u32 = 0;
    let mut i: usize = 0;
    let mmatch: u8x64 = u8x64::splat(b'm');
    let vmatch: u8x4 = u8x4::from([b'm', b'u', b'l', b'(']);
    while i < bl - 64 {
        let curr: Simd<u8, 64> = u8x64::load_or_default(&b[i .. i + 64]);
        let x = curr.simd_eq(mmatch);
        if !x.any() {
            i += 64;
            continue;
        }
        let mut bits = x.to_bitmask().reverse_bits();
        while bits != 0 {
            let pos = bits.trailing_zeros() as usize;
            let match_at: Mask<i8, 4> = u8x4::load_or_default(&b[i + 63 - pos..i + 67 - pos]).simd_eq(vmatch);
            if match_at.all() {
                let mut num1: u32 = 0;
                let mut j: usize = i + 67 - pos;
                while b[j] >= b'0' && b[j] <= b'9' {
                    num1 *= 10;
                    num1 += u32::from(b[j].wrapping_sub(b'0'));
                    j += 1;
                }
                if num1 == 0 || b[j] != b',' {
                    bits &= !((1u64 << j - (i + 67 - pos)) - 1) << pos;
                    continue;
                }
                j += 1;
                let mut num2: u32 = 0;
                while b[j] >= b'0' && b[j] <= b'9' {
                    num2 *= 10;
                    num2 += u32::from(b[j].wrapping_sub(b'0'));
                    j += 1;
                }
                if num2 == 0 || b[j] != b')' {
                    bits &= !((1u64 << j - (i + 67 - pos)) - 1) << pos;
                    continue;
                }
                sum += num1 * num2;
                bits &= !0xffu64 << pos;
            } else { bits = bits & (bits - 1); } 
        }
        i += 64;
    }
    while i < bl - 8 {
        i += 1;
        if b[i - 1] != b'm' {
        } else {
            if b[i] != b'u' || b[i + 1] != b'l' || b[i + 2] != b'(' {
                continue;
            }
            let mut num1: u32 = 0;
            i += 3;
            while i < bl && b[i] >= b'0' && b[i] <= b'9' {
                num1 *= 10;
                num1 += u32::from(b[i].wrapping_sub(b'0'));
                i += 1;
            }
            if num1 == 0 || i == bl || b[i] != b',' { continue; }
            i += 1;
            let mut num2: u32 = 0;
            while i < bl && b[i] >= b'0' && b[i] <= b'9' {
                num2 *= 10;
                num2 += u32::from(b[i].wrapping_sub(b'0'));
                i += 1;
            }
            if num2 == 0 || i == bl || b[i] != b')' { continue; }
            i += 1;
            sum += num1 * num2;
        }
    }
    return sum;
}

pub fn part2(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();
    let mut sum = 0;
    let mut i = 0;
    let mut should_do = true;
    while i < bl - 8 {
        if b[i] == b'd' {
            i += 1;
            if b[i] == b'o' {
                let flip: bool = if should_do { b[i + 1] == b'n' && b[i + 2] == b'\'' && b[i + 3] == b't' && b[i + 4] == b'(' && b[i + 5] == b')' }
                else { b[i + 1] == b'(' && b[i + 2] == b')' };
                i += if !flip {1} else { if should_do {5} else {2}};
                should_do ^= flip;
            }
        } else if !should_do || b[i] != b'm' {
            i += 1;
        } else {
            i += 1;
            if b[i] != b'u' || b[i + 1] != b'l' || b[i + 2] != b'(' {
                continue;
            }
            let mut num1: u32 = 0;
            i += 3;
            while i < bl && b[i] >= b'0' && b[i] <= b'9' {
                num1 *= 10;
                num1 += u32::from(b[i].wrapping_sub(b'0'));
                i += 1;
            }
            if num1 == 0 || i == bl || b[i] != b',' { continue; }
            i += 1;
            let mut num2: u32 = 0;
            while i < bl && b[i] >= b'0' && b[i] <= b'9' {
                num2 *= 10;
                num2 += u32::from(b[i].wrapping_sub(b'0'));
                i += 1;
            }
            if num2 == 0 || i == bl || b[i] != b')' { continue; }
            i += 1;
            sum += num1 * num2;
        }
    }
    return sum;
}