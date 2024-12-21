use std::simd::prelude::*;

const W: usize = 141;
const W_L: usize = W + 1;
const PART1_THRESH: u16 = 100;

pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut vis = [u16::MAX; W_L * (W + 2) + 32];
        let start = memchr::memchr(b'S', b).unwrap_unchecked();
        let mut pos = start;
        *vis.get_unchecked_mut(pos) = 0;
        let mut depth = 1u16;
        let mut prev_d = -1;
        loop {
            if prev_d != 1 {
                while *b.get_unchecked(pos + W_L) != b'#' {
                    prev_d = 1;
                    pos += W_L;
                    *vis.get_unchecked_mut(pos) = depth;
                    depth += 1;
                }
            }
            if prev_d != 0 {
                while *b.get_unchecked(pos + 1) != b'#' {
                    prev_d = 0;
                    pos += 1;
                    *vis.get_unchecked_mut(pos) = depth;
                    depth += 1;
                }
            }
            if prev_d != 0 {
                while *b.get_unchecked(pos - 1) != b'#' {
                    prev_d = 0;
                    pos -= 1;
                    *vis.get_unchecked_mut(pos) = depth;
                    depth += 1;
                }
            }
            if prev_d != 1 {
                while *b.get_unchecked(pos - W_L) != b'#' {
                    prev_d = 1;
                    pos -= W_L;
                    *vis.get_unchecked_mut(pos) = depth;
                    depth += 1;
                }
            }
            if *b.get_unchecked(pos) == b'E' { break; }
        }
        let mut ans = 0u32;
        let mut ptr = 0;
        while ptr < bl - 3 {
            let curr = u16x32::load_select_unchecked(
                &vis[ptr..ptr+32],
                mask16x32::splat(true),
                u16x32::splat(0));
            let other = u16x32::load_select_unchecked(
                &vis[ptr+2..ptr+34],
                mask16x32::splat(true),
                u16x32::splat(0));
            let upper = curr.simd_max(other);
            let lower = curr.simd_min(other);
            let valid = upper.simd_lt(u16x32::splat(u16::MAX));
            ans += ((upper - lower).simd_ge(u16x32::splat(PART1_THRESH+2)) & valid).to_bitmask().count_ones();
            let other = u16x32::load_select_unchecked(
                &vis[ptr+2*W_L..ptr+2*W_L+32],
                mask16x32::splat(true),
                u16x32::splat(0));
            let upper = curr.simd_max(other);
            let lower = curr.simd_min(other);
            let valid = upper.simd_lt(u16x32::splat(u16::MAX));
            ans += ((upper - lower).simd_ge(u16x32::splat(PART1_THRESH+2)) & valid).to_bitmask().count_ones();
            ptr += 32;
        }
        return ans;
    }
}

pub fn part2(_s: &str) -> u32 {
    0
}