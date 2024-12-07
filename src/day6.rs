use core::simd::prelude::*;

pub unsafe fn _part1(b: &[u8]) -> u32 {
    let bl = b.len();
    let w = memchr::memchr(b'\n', b).unwrap_unchecked() + 1;
    let h = bl / w;
    let it_per_row = w / 64;
    let ex_per_row = w % 64;
    let n = (h + 2) * (w + 63) / 64;
    let mut visited = vec![0u64; n];
    let mut obstacles = vec![0u64; n];
    let start = memchr::memchr(b'^', b).unwrap_unchecked();
    {
        let mut src = 0;
        let mut dst = it_per_row + 1;
        for _ in 0..h {
            for _ in 0..it_per_row {
                let obs = u8x64::from_slice(&b[src..src+64]).simd_eq(u8x64::splat(b'#')).to_bitmask();
                obstacles[dst] = obs;
                dst += 1;
                src += 64;
            }
            for j in 0..ex_per_row {
                if b[src] == b'#' {
                    obstacles[dst] |= 1 << j;
                }
                src += 1;
            }
            dst += 1;
        }
    }
    let x = start % w;
    let y = start / w + 1;
    let mut xmask: u64 = 1 << (x & 63);
    let mut ptr = y * (it_per_row + 1) + (x / 64);
    'outer:
    loop {
        // println!("U: {} {}",  ptr / (it_per_row + 1), (ptr % (it_per_row + 1) * 64) as u32 + xmask.trailing_zeros());
        loop {
            visited[ptr] |= xmask;
            let next = ptr - it_per_row - 1;
            if next <= it_per_row { break 'outer; }
            if obstacles[next] & xmask != 0 { break; }
            ptr = next;
        }
        // println!("R: {} {}",  ptr / (it_per_row + 1), (ptr % (it_per_row + 1) * 64) as u32 + xmask.trailing_zeros());
        {
            let obs = obstacles[ptr];
            let chunk = obs.wrapping_sub(xmask);
            let next_xmask = obs & !chunk;
            visited[ptr] |= next_xmask.wrapping_sub(xmask);
            if next_xmask != 0 {
                xmask = next_xmask >> 1;
            } else {
                loop {
                    if (ptr + 1) % (it_per_row + 1) == 0 { break 'outer; }
                    ptr += 1;
                    if obstacles[ptr] != 0 {
                        let chunk = obstacles[ptr] - 1;
                        xmask = (obstacles[ptr] & !chunk) >> 1;
                        if xmask != 0 {
                            visited[ptr] |= xmask | xmask - 1;
                        } else {
                            ptr -= 1;
                            xmask = 1 << 63;
                        }
                        break;
                    }
                    visited[ptr] = u64::MAX;
                }
            }
        }
        // println!("D: {} {}",  ptr / (it_per_row + 1), (ptr % (it_per_row + 1) * 64) as u32 + xmask.trailing_zeros());
        loop {
            visited[ptr] |= xmask;
            let next = ptr + it_per_row + 1;
            if next >= n - it_per_row - 1 { break 'outer; }
            if obstacles[next] & xmask != 0 { break; }
            ptr = next;
        }
        // println!("L: {} {}", ptr / (it_per_row + 1), (ptr % (it_per_row + 1) * 64) as u32 + xmask.trailing_zeros());
        {
            if obstacles[ptr] & (xmask - 1) != 0 {
                let next_xmask = 1 << (64 - (obstacles[ptr] & (xmask - 1)).leading_zeros());
                visited[ptr] |= (xmask - next_xmask) | xmask;
                xmask = next_xmask;
            } else {
                visited[ptr] |= xmask | xmask - 1;
                loop {
                    if ptr % (it_per_row + 1) == 0 { break 'outer; }
                    ptr -= 1;
                    if obstacles[ptr] != 0 {
                        let lead = obstacles[ptr].leading_zeros();
                        if lead != 0 { // If obstacle is at max bit of word
                            let next_xmask = 1 << (64 - obstacles[ptr].leading_zeros());
                            visited[ptr] |= 0u64.wrapping_sub(next_xmask);
                            xmask = next_xmask;
                        } else {
                            ptr += 1;
                            xmask = 1;
                        }
                        break;
                    }
                    visited[ptr] = u64::MAX;
                }
            }
        }
    }
    let mut ans = 0;
    {
        let mut ptr = it_per_row + 1;
        for _ in 0..h {
            for _ in 0..it_per_row {
                ans += visited.get_unchecked(ptr).count_ones();
                ptr += 1;
            }
            ans += (visited[ptr] & ((1 << (ex_per_row - 1)) - 1)).count_ones();
            ptr += 1;

        }
    }
    return ans;
}

pub fn part1(s: &str) -> u32 {
    unsafe {
        return _part1(s.as_bytes());
    }
}
pub fn part2(_s: &str) -> u32 {
    0
}
