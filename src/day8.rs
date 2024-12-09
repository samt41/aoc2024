use core::simd::prelude::*;

const W: u16 = 51;
pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut antinodes = [0u64; 64];
        let mut nodes = [0u64; 128];
        let mut y = 0u32;
        for x in (0..bl).step_by(W as usize) {
            let map = u8x64::load_select_unchecked(
                &b[x..x+W as usize],
                mask8x64::from_bitmask((1 << (W - 1)) - 1),
                u8x64::splat(0));
            let mut mask = map.simd_ge(u8x64::splat(b'0')).to_bitmask() as u64;
            while mask != 0 {
                let pos = mask.trailing_zeros() as usize;
                let realx = (b[x + pos] - b'0') as usize;
                nodes[realx] = (nodes[realx] << 16) | 32768 | y as u64 | pos as u64;
                mask = mask & (mask - 1);
            }
            y += 256;
        }
        for i in 0..75 {
            let curr = nodes[i];
            let n = (79 - curr.leading_zeros()) / 16;
            for j in 0..n {
                let nj = (curr >> (j * 16)) as u16 & 0x7fff;
                for k in 0..j {
                    let nk = (curr >> (k * 16)) as u16 & 0x7fff;
                    let xy = nk - nj;
                    let antinode1 = nj.wrapping_sub(xy);
                    let antinode2 = nk + xy;
                    if antinode1 & 0xC0C0 == 0 {
                        antinodes[(antinode1 >> 8) as usize] |= 1 << (antinode1 & 63);
                    }
                    if antinode2 & 0xC0C0 == 0 {
                        antinodes[(antinode2 >> 8) as usize] |= 1 << (antinode2 & 63);
                    }
                }
            }
        }
        let mut ans = 0;
        for i in 0..(W as usize - 1) {
            ans += (antinodes[i] & ((1 << (W - 1)) - 1)).count_ones();
        }
        return ans;
    }
}

pub fn part2(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut antinodes = [0u64; 64];
        let mut nodes = [0u64; 128];
        let mut y = 0u32;
        for x in (0..bl).step_by(W as usize) {
            let map = u8x64::load_select_unchecked(
                &b[x..x+W as usize],
                mask8x64::from_bitmask((1 << (W - 1)) - 1),
                u8x64::splat(0));
            let mut mask = map.simd_ge(u8x64::splat(b'0')).to_bitmask() as u64;
            while mask != 0 {
                let pos = mask.trailing_zeros() as usize;
                let realx = (b[x + pos] - b'0') as usize;
                nodes[realx] = (nodes[realx] << 16) | 32768 | y as u64 | pos as u64;
                mask = mask & (mask - 1);
            }
            y += 256;
        }
        for i in 0..75 {
            let curr = nodes[i];
            let n = (79 - curr.leading_zeros()) / 16;
            for j in 0..n {
                let nj = (curr >> (j * 16)) as u16 & 0x7fff;
                for k in 0..j {
                    let nk = (curr >> (k * 16)) as u16 & 0x7fff;
                    let xy = nk - nj;
                    let mut antinode = nj;
                    loop {
                        if antinode & 0xC0C0 == 0 {
                            antinodes[(antinode >> 8) as usize] |= 1 << (antinode & 63);
                        } else { break; }
                        antinode = antinode.wrapping_sub(xy);
                    }
                    antinode = nk;
                    loop {
                        if antinode & 0xC0C0 == 0 {
                            antinodes[(antinode >> 8) as usize] |= 1 << (antinode & 63);
                        } else { break; }
                        antinode += xy;
                    }
                }
            }
        }
        let mut ans = 0;
        for i in 0..(W as usize - 1) {
            ans += (antinodes[i] & ((1 << (W - 1)) - 1)).count_ones();
        }
        return ans;
    }
}
