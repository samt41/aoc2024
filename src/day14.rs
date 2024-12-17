use core::simd::prelude::*;

const W16: u16 = 101;
const H16: u16 = 103;
pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut ptr = 2;
        let mut buf = [0u8; 128];
        let mut q1 = 0u32;
        let mut q2 = 0u32;
        let mut q3 = 0u32;
        let mut q4 = 0u32;
        let mut robots = 500;
        while robots >= 32 {
            for r in 0..32 {
                let (px, p) = atoi_simd::parse_any_pos::<u8>(&b[ptr..ptr+3]).unwrap_unchecked();
                ptr += p + 1;
                let (py, p) = atoi_simd::parse_any_pos::<u8>(&b[ptr..ptr+3]).unwrap_unchecked();
                ptr += p + 3;
                let (vx, p) = atoi_simd::parse_any::<i8>(&b[ptr..ptr+4]).unwrap_unchecked();
                ptr += p + 1;
                let (vy, p) = atoi_simd::parse_any::<i8>(&b[ptr..ptr+4]).unwrap_unchecked();
                ptr += p + 3;
                buf[ 0 + r] = px;
                buf[32 + r] = py;
                buf[64 + r] = if vx < 0 { vx + W16 as i8 } else { vx } as u8;
                buf[96 + r] = if vy < 0 { vy + H16 as i8 } else { vy } as u8;
            }
            robots -= 32;
            let px = u8x32::load_select_unchecked(&buf[ 0.. 32], mask8x32::splat(true), u8x32::splat(0)).cast::<u16>();
            let py = u8x32::load_select_unchecked(&buf[32.. 64], mask8x32::splat(true), u8x32::splat(0)).cast::<u16>();
            let vx = u8x32::load_select_unchecked(&buf[64.. 96], mask8x32::splat(true), u8x32::splat(0)).cast::<u16>();
            let vy = u8x32::load_select_unchecked(&buf[96..128], mask8x32::splat(true), u8x32::splat(0)).cast::<u16>();
            let x = (px + vx * u16x32::splat(100)) % u16x32::splat(W16);
            let y = (py + vy * u16x32::splat(100)) % u16x32::splat(H16);
            let xle = x.simd_lt(u16x32::splat(W16 / 2)).to_bitmask();
            let yle = y.simd_lt(u16x32::splat(H16 / 2)).to_bitmask();
            let xge = x.simd_gt(u16x32::splat(W16 / 2)).to_bitmask();
            let yge = y.simd_gt(u16x32::splat(H16 / 2)).to_bitmask();
            q1 += (xle & yle).count_ones();
            q2 += (xle & yge).count_ones();
            q3 += (xge & yle).count_ones();
            q4 += (xge & yge).count_ones();
        }
        while ptr < bl {
            let (px, p) = atoi_simd::parse_any_pos::<u8>(&b[ptr..ptr+3]).unwrap_unchecked();
            ptr += p + 1;
            let (py, p) = atoi_simd::parse_any_pos::<u8>(&b[ptr..ptr+3]).unwrap_unchecked();
            ptr += p + 3;
            let (vx, p) = atoi_simd::parse_any::<i8>(&b[ptr..ptr+4]).unwrap_unchecked();
            ptr += p + 1;
            let (vy, p) = atoi_simd::parse_any::<i8>(&b[ptr..]).unwrap_unchecked();
            ptr += p + 3;
            let x = (px as u16 + (if vx < 0 { vx + W16 as i8 } else { vx } as u16) * 100) % W16;
            let y = (py as u16 + (if vy < 0 { vy + H16 as i8 } else { vy } as u16) * 100) % H16;
            let xle = x < W16 / 2;
            let xge = x > W16 / 2;
            let yle = y < H16 / 2;
            let yge = y > H16 / 2;
            if xle {
                if yle {
                    q1 += 1;
                } else if yge {
                    q2 += 1;
                }
            } else if xge {
                if yle {
                    q3 += 1;
                } else if yge {
                    q4 += 1;
                }
            }
        }
        return q1 * q2 * q3 * q4;
    }
}

pub fn part2(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut ptr = 2;
        let mut vis = [0u128; 256];
        while ptr < bl {
            let (px, p) = atoi_simd::parse_any_pos::<u8>(&b[ptr..ptr+3]).unwrap_unchecked();
            ptr += p + 1;
            let (py, p) = atoi_simd::parse_any_pos::<u8>(&b[ptr..ptr+3]).unwrap_unchecked();
            ptr += p + 3;
            let (vx, p) = atoi_simd::parse_any::<i8>(&b[ptr..ptr+4]).unwrap_unchecked();
            ptr += p + 1;
            let (vy, p) = atoi_simd::parse_any::<i8>(&b[ptr..]).unwrap_unchecked();
            ptr += p + 3;
            vis[if vx < 0 { vx + W16 as i8 } else { vx } as usize] |= 1 << px;
            vis[128 + if vy < 0 { vy + W16 as i8 } else { vy } as usize] |= 1 << py;
        }
        let mut min_cx = 129;
        let mut min_cy = 129;
        let mut cx = 0i16;
        let mut cy = 0i16;
        for t in 0..W16.max(H16){
            let mut countx = 0u128;
            let mut county = 0u128;

            for addr in 0..W16 {
                countx |= vis[addr as usize];
                vis[addr as usize] = (vis[addr as usize] >> addr) | ((vis[addr as usize] & ((1 << addr) - 1)) << (W16 - addr));
            }
            for addr in 0..H16 {
                county |= vis[128 + addr as usize];
                vis[128 + addr as usize] = (vis[128 + addr as usize] >> addr) | ((vis[128 + addr as usize] & ((1 << addr) - 1)) << (H16 - addr));
            }
            let xx = countx.count_ones();
            let yy = county.count_ones();
            if xx < min_cx {
                min_cx = xx;
                cx = W16 as i16 - t as i16;
            }
            if yy < min_cy {
                min_cy = yy;
                cy = H16 as i16 - t as i16;
            }
        }
        return (cx as u16 + ((51 * (cy - cx)).rem_euclid(H16 as i16)) as u16 * W16) as u32;
    }
}