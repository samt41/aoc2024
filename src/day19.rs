use std::{collections::HashSet, simd::u8x64};
use std::simd::prelude::*;

pub fn part1(s: &str) -> u32 {
    unsafe {
        let mut keys = HashSet::<u64>::new();
        let b = s.as_bytes();
        let bl = b.len();
        let mut ptr = 0;
        let mut dp = [false; 128];
        'read_keys:
        loop {
            let line = u8x64::load_select_unchecked(&b[ptr..ptr+64], mask8x64::splat(true), u8x64::splat(0));
            let mut seps = line.simd_lt(u8x64::splat(b'a')).to_bitmask();
            let mut prev_pos = 0;
            while seps != 0 {
                let pos = seps.trailing_zeros() as usize;
                let mut word = u64::from_be_bytes((&b[ptr+prev_pos..ptr+prev_pos+8]).try_into().unwrap_unchecked());
                let dst = (pos - prev_pos) as u32;
                if dst > 8 {
                    ptr += prev_pos;
                    break 'read_keys;
                }
                word >>= 64 - (dst << 3);
                keys.insert(word);
                prev_pos = pos + 2;
                seps &= seps.wrapping_sub(1);
                seps &= seps.wrapping_sub(1);
            }
            ptr += prev_pos;
        }
        let mut it = memchr::memchr_iter(b'\n', &b[ptr..]);
        let mut left = ptr;
        let mut res = 0u32;
        while left < bl {
            let right = ptr + it.next().unwrap_unchecked();
            dp.fill(false);
            dp[0] = true;
            let mut window = 0u64;
            for i in 1..right-left+1 {
                window = (window << 8) | b[left + i - 1] as u64;
                let mut mask: u64 = 0xff;
                for j in ((i as i32 - 8).max(0)..i as i32).rev() {
                    let window2 = window & mask;
                    mask |= mask << 8;
                    if keys.contains(&window2) {
                        dp[i] |= dp[j as usize];
                    }
                }
            }
            res += dp[right-left] as u32;
            left = right + 1;
        }
        return res;
    }
}

pub fn part2(s: &str) -> u64 {
    unsafe {
        let mut keys = HashSet::<u64>::new();
        let b = s.as_bytes();
        let bl = b.len();
        let mut ptr = 0;
        let mut dp = [0u64; 128];
        'read_keys:
        loop {
            let line = u8x64::load_select_unchecked(&b[ptr..ptr+64], mask8x64::splat(true), u8x64::splat(0));
            let mut seps = line.simd_lt(u8x64::splat(b'a')).to_bitmask();
            let mut prev_pos = 0;
            while seps != 0 {
                let pos = seps.trailing_zeros() as usize;
                let dst = (pos - prev_pos) as u32;
                if dst > 8 {
                    ptr += prev_pos;
                    break 'read_keys;
                }
                let mut word = u64::from_be_bytes((&b[ptr+prev_pos..ptr+prev_pos+8]).try_into().unwrap_unchecked());
                word >>= 64 - (dst << 3);
                // word &= (1u64.unbounded_shl(dst << 3)).wrapping_sub(1);
                keys.insert(word);
                prev_pos = pos + 2;
                seps &= seps.wrapping_sub(1);
                seps &= seps.wrapping_sub(1);
            }
            ptr += prev_pos;
        }
        let mut it = memchr::memchr_iter(b'\n', &b[ptr..]);
        let mut left = ptr;
        let mut res = 0u64;
        while left < bl {
            let right = ptr + it.next().unwrap_unchecked();
            dp.fill(0);
            dp[0] = 1;
            let mut window = 0u64;
            for i in 1..right-left+1 {
                window = (window << 8) | b[left + i - 1] as u64;
                let mut mask: u64 = 0xff;
                for j in ((i as i32 - 8).max(0)..i as i32).rev() {
                    let window2 = window & mask;
                    mask |= mask << 8;
                    if keys.contains(&window2) {
                        dp[i] += dp[j as usize];
                    }
                }
            }
            res += dp[right-left];
            left = right + 1;
        }
        return res;
    }
}
