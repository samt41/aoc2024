use core::simd::prelude::*;

const W: u32 = 51;
pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut antinodes = vec![0u8; bl];
        let mut nodes = vec![0u32];
        let mut y = 0u32;
        for x in (0..bl).step_by(W as usize) {
            let map = u8x64::load_select_unchecked(
                &b[x..x+W as usize],
                mask8x64::from_bitmask((1 << (W - 1)) - 1),
                u8x64::splat(0));
            let mut mask = map.simd_ge(u8x64::splat(b'0')).to_bitmask() as u64;
            while mask != 0 {
                let pos = mask.trailing_zeros() as usize;
                nodes.push(((b[x + pos] as u32) << 16) | (y << 8) | pos as u32);
                mask = mask & (mask - 1);
            }
            y += 1;
        }
        nodes.sort_unstable();
        let mut start = 0;
        for end in 1..nodes.len() {
            let ne = nodes[end];
            if (ne ^ nodes[start]) >> 16 != 0 {
                start = end;
                continue;
            }
            for i in start..end {
                let nc = nodes[i];
                let xy = ne - nc;
                let antinode1 = nc - xy;
                let antinode2 = ne + xy;
                if antinode1 & 0xff < W - 1{
                    let pos = (((antinode1 & 0xff00) * W >> 8) + (antinode1 & 0xff)) as usize;
                    if pos < bl { antinodes[pos] = 1; }
                }
                if antinode2 & 0xff < W - 1 {
                    let pos = (((antinode2 & 0xff00) * W >> 8) + (antinode2 & 0xff)) as usize;
                    if pos < bl { antinodes[pos] = 1; }
                }
            }
        }
        let mut ans = 0;
        for i in (0..bl).step_by(64) {
            ans += u8x64::load_or_default(&antinodes[i..usize::min(i+64, bl)]).simd_eq(u8x64::splat(1)).to_bitmask().count_ones();
        }
        return ans;
    }
}

pub fn part2(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut antinodes = vec![0u8; bl];
        let mut nodes = vec![0u32];
        let mut y = 0u32;
        for x in (0..bl).step_by(W as usize) {
            let map = u8x64::load_select_unchecked(
                &b[x..x+W as usize],
                mask8x64::from_bitmask((1 << (W - 1)) - 1),
                u8x64::splat(0));
            let mut mask = map.simd_ge(u8x64::splat(b'0')).to_bitmask() as u64;
            while mask != 0 {
                let pos = mask.trailing_zeros() as usize;
                nodes.push(((b[x + pos] as u32) << 16) | (y << 8) | pos as u32);
                mask = mask & (mask - 1);
            }
            y += 1;
        }
        nodes.sort_unstable();
        let mut start = 0;
        for end in 1..nodes.len() {
            let ne = nodes[end];
            if (ne ^ nodes[start]) >> 16 != 0 {
                start = end;
                continue;
            }
            for i in start..end {
                let nc = nodes[i];
                let xy = ne - nc;
                let mut antinode = nc;
                loop {
                    let pos = (((antinode & 0xff00) * W >> 8) + (antinode & 0xff)) as usize;
                    let valid = antinode & 0xff < W - 1 && pos < bl;
                    if valid { antinodes[pos] = 1; }
                    else { break; }
                    antinode -= xy;
                }
                antinode = ne;
                loop {
                    let pos = (((antinode & 0xff00) * W >> 8) + (antinode & 0xff)) as usize;
                    let valid = antinode & 0xff < W - 1 && pos < bl;
                    if valid { antinodes[pos] = 1; }
                    else { break; }
                    antinode += xy;
                }
            }
        }
        let mut ans = 0;
        for i in (0..bl).step_by(64) {
            ans += u8x64::load_or_default(&antinodes[i..usize::min(i+64, bl)]).simd_eq(u8x64::splat(1)).to_bitmask().count_ones();
        }
        return ans;
    }
}
