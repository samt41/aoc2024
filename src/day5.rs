use std::cmp::Ordering;
use core::simd::prelude::*;

pub fn part1(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();

    let muls = u8x64::from_array([
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0, 0,
    ]);
    let mut masks = [0 as u128; 128];
    let mut pos = 0;
    {
        while pos + 64 < bl {
            let line = u8x64::load_or_default(&b[pos..pos+64]);
            let linenorm = (line - u8x64::splat(b'0')) * muls;
            let lineadd = linenorm + linenorm.rotate_elements_left::<1>();
            let npos = lineadd.simd_eq(u8x64::splat(132)).first_set();
            let nposw = npos.unwrap_or(60);

            let nums = lineadd.as_array();
            for i in (0..nposw).step_by(6) {
                masks[nums[i] as usize] |= (1u128) << nums[i + 3];
            }
            pos += nposw;
            if npos.is_some() { break; }
        }
    }
    pos += 2;
    let mut ans = 0;
    {
        let mut mask: u128 = 0u128;
        let mut valid: bool = true;
        let mut prev_pos: usize = pos;
        while pos + 2 < bl {
            if valid {
                let page: u8 = (b[pos] - b'0') * 10 + b[pos + 1] - b'0';
                valid &= masks[page as usize] & mask == 0;
                mask |= 1 << page;
            }
            if b[pos + 2] == b'\n' {
                mask = 0;
                if valid {
                    ans += ((b[prev_pos + (pos + 2 - prev_pos) / 2 - 1] - b'0') * 10 + b[prev_pos + (pos + 2 - prev_pos) / 2] - b'0') as u32
                }
                valid = true;
                prev_pos = pos + 3;
            }
            pos += 3;
        }
    }
    return ans;
}

pub fn part2(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();

    let muls = u8x64::from_array([
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0,
        10, 1, 0, 0,
    ]);
    let mut masks = [0 as u128; 128];
    let mut pos = 0;
    {
        while pos + 64 < bl {
            let line = u8x64::load_or_default(&b[pos..pos+64]);
            let linenorm = (line - u8x64::splat(b'0')) * muls;
            let lineadd = linenorm + linenorm.rotate_elements_left::<1>();
            let npos = lineadd.simd_eq(u8x64::splat(132)).first_set();
            let nposw = npos.unwrap_or(60);

            let nums = lineadd.as_array();
            for i in (0..nposw).step_by(6) {
                masks[nums[i] as usize] |= (1u128) << nums[i + 3];
            }
            pos += nposw;
            if npos.is_some() { break; }
        }
    }
    pos += 2;
    let mut ans = 0;
    {
        let mut buf = vec![0 as u16; 32];
        buf.clear();
        let mut mask: u128 = 0u128;
        let mut valid: bool = true;
        while pos + 2 < bl {
            let page: u16 = ((b[pos] - b'0') * 10 + b[pos + 1] - b'0') as u16;
            buf.push(page | (buf.len() << 8) as u16);
            valid &= masks[page as usize] & mask == 0;
            mask |= 1 << page;
            if b[pos + 2] == b'\n' {
                if !valid {
                    let num_items = buf.len();
                    let (_a, b, _c) = buf.select_nth_unstable_by(num_items / 2, |a, b| {
                        return if (masks[(*b & 0xff) as usize] & (1u128 << (*a & 0xff))) == 0 {
                            if *a > *b { Ordering::Greater } else { Ordering::Equal }
                        } else { Ordering::Less };
                    });
                    ans += (*b & 0xff) as u32;
                }
                mask = 0;
                valid = true;
                buf.clear();
            }
            pos += 3;
        }
    }
    return ans;
}
