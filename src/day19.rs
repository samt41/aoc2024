use std::{collections::HashSet, simd::u8x64};
use std::simd::prelude::*;

pub fn part1(s: &str) -> u32 {
    unsafe {
        let mut keys = HashSet::<u32>::new();
        keys.reserve(2048);
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
                word |= (word & 0x1010101010101010) >> 1;
                let mut word1 = (word as u32) & 0x0f0f0f0f;
                let mut word2 = ((word >> 32) as u32) & 0x0f0f0f0f;
                word1 = ((word1 & 0x0f000f00) >> 4) | (word1 & 0x000f000f);
                word1 = ((word1 & 0x00ff0000) >> 8) | (word1 & 0x000000ff);
                word2 = ((word2 & 0x0f000f00) << 4) | ((word2 & 0x000f000f) << 8);
                word2 = ((word2 & 0x0000ff00) << 8) | (word2 & 0xff000000);
                keys.insert(word1 | word2);
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
            let mut window = 0u32;
            for i in 1..right-left+1 {
                let mut next = b[left + i - 1];
                next |= (next & 0x10) >> 1;
                window = (window << 4) | (next & 0xf) as u32;
                let mut mask: u32 = 0xf;
                for j in ((i as i32 - 8).max(0)..i as i32).rev() {
                    let window2 = window & mask;
                    mask |= mask << 4;
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
        let mut keys = HashSet::<u32>::new();
        keys.reserve(2048);
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
                word |= (word & 0x1010101010101010) >> 1;
                let mut word1 = (word as u32) & 0x0f0f0f0f;
                let mut word2 = ((word >> 32) as u32) & 0x0f0f0f0f;
                word1 = ((word1 & 0x0f000f00) >> 4) | (word1 & 0x000f000f);
                word1 = ((word1 & 0x00ff0000) >> 8) | (word1 & 0x000000ff);
                word2 = ((word2 & 0x0f000f00) << 4) | ((word2 & 0x000f000f) << 8);
                word2 = ((word2 & 0x0000ff00) << 8) | (word2 & 0xff000000);
                keys.insert(word1 | word2);
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
            let mut window = 0u32;
            for i in 1..right-left+1 {
                let mut next = b[left + i - 1];
                next |= (next & 0x10) >> 1;
                window = (window << 4) | (next & 0xf) as u32;
                let mut mask: u32 = 0xf;
                for j in ((i as i32 - 8).max(0)..i as i32).rev() {
                    let window2 = window & mask;
                    mask |= mask << 4;
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
