use core::simd::prelude::*;
use std::intrinsics::simd::simd_select;

#[derive(Clone, Copy)]
struct GcdObj {
    gcd: u8,
    A: i8,
    B: i8,
    _pad: u8
}

const fn build_gcd(a: i8, b: i8, x: &mut i8, y: &mut i8) -> i8 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let mut x1: i8 = 0;
    let mut y1: i8 = 0;
    let d: i8 = build_gcd(b, a % b, &mut x1, &mut y1);
    *x = y1;
    *y = (x1 as i16 - y1 as i16 * (a / b) as i16) as i8;
    return d;
}

static GCD: [GcdObj; 10000] = {
    let mut arr = [GcdObj{gcd: 0, A: 0, B: 0, _pad: 0}; 10000];
    let mut i = 0;
    while i < 10000 {
        let mut x = 0i8;
        let mut y = 0i8;
        arr[i].gcd = build_gcd((i / 100) as i8, (i % 100) as i8, &mut x, &mut y) as u8;
        arr[i].A = x;
        arr[i].B = y;
        i += 1;
    }
    arr
};

unsafe fn calc1(data: u8x64, targets: u16x32) -> u32 {
    let zeros = i32x16::splat(0);
    let bignum = i32x16::splat(i32::MAX);
    let hundredsu = u32x16::splat(100);
    let x1 = data.resize::<16>(0).cast::<i32>();
    let y1 = data.rotate_elements_left::<16>().resize::<16>(0).cast::<i32>();
    let x2 = data.rotate_elements_left::<32>().resize::<16>(0).cast::<i32>();
    let y2 = data.rotate_elements_left::<48>().resize::<16>(0).cast::<i32>();
    let X = targets.resize::<16>(0).cast::<i32>();
    let Y = targets.rotate_elements_left::<16>().resize::<16>(0).cast::<i32>();
    let det = x1 * y2 - x2 * y1;
    let sol1 = det.simd_ne(zeros);
    let det_clean = simd_select(sol1.to_int(), det, bignum);
    // unique solution
    let solA = X * y2 - Y * x2;
    let solB = Y * x1 - X * y1;

    let A_int = solA / det_clean;
    let B_int = solB / det_clean;
    let sol1_is_integer = (A_int * det).simd_eq(solA) & (B_int * det).simd_eq(solB);
    let sol1_in_range = A_int.cast::<u32>().simd_le(hundredsu) & B_int.cast::<u32>().simd_le(hundredsu);
    let sol1_valid = (sol1_is_integer & sol1_in_range).to_int();

    let A1 = simd_select(sol1_valid, A_int, zeros);
    let B1 = simd_select(sol1_valid, B_int, zeros);

    let mut ans = (A1 * i32x16::splat(3) + B1).reduce_sum() as u32;

    if sol1.all() {
        return ans;
    }
    // no solution
    let sol0 = (x1 * Y - y1 * X).simd_ne(zeros);

    // infinite solutions (collinear)
    let mut solinf = (!sol0 & !sol1).to_bitmask();
    if solinf == 0 { return ans; }
    else {
        // disaster.
        while solinf != 0 {
            let idx = solinf.trailing_zeros() as usize;
            let x1_ = x1[idx];
            let x2_ = x2[idx];
            let X_ = X[idx];
            
            let gcd_ = GCD[(x1_ * 100 + x2_) as usize];
            let gcd = gcd_.gcd as i32;
            let div_ = X_ / gcd;
            let partXA = gcd_.A as i32 * div_;
            let partXB = gcd_.B as i32 * div_;
            let sclA = x2_ / gcd;
            let sclB = x1_ / gcd;
            let k = if x1_ >= x2_ * 3 { (partXB - partXB.rem_euclid(sclB)) / sclB } // Maximize A
                else { -(partXA - partXA.rem_euclid(sclA)) / sclA }; // Maximize B
            let A_final = partXA + k * sclA;
            let B_final = partXB - k * sclB;
            if (A_final as u32) <= 100 && (B_final as u32) <= 100 {
                ans += (A_final * 3 + B_final) as u32;
            }
            solinf &= solinf - 1;
        }
    }
    return ans;
}

pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut ptr = 0;
        let mut buf = [0u8; 128];
        let mut buf_target = [0u16; 32];
        let mut ans = 0u32;
        let _load_idxs = usizex8::from_array([0, 1, 6, 7, 21, 22, 27, 28]);
        let _buf_idxs = usizex8::from_array([0,64,16,80,32,96,48,112]);
        while ptr < bl {
            for buf_ptr in 0..16 {
                ptr += 12;
                let stuff = u8x8::gather_select_unchecked(&b[ptr..ptr + 32], masksizex8::splat(true), _load_idxs, u8x8::splat(0));
                stuff.scatter_select_unchecked(&mut buf[buf_ptr..], masksizex8::splat(true), _buf_idxs);
                ptr += 39;
                let (num1, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
                ptr += len + 4;
                let (num2, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
                ptr += len + 2;
                buf_target[buf_ptr] = num1;
                buf_target[16 + buf_ptr] = num2;
            }
            let word1 = u8x64::load_select_unchecked(&buf[0 ..64 ], mask8x64::splat(true), u8x64::splat(0));
            let word2 = u8x64::load_select_unchecked(&buf[64..128], mask8x64::splat(true), u8x64::splat(0));
            let data = word1 * u8x64::splat(10) + word2 - u8x64::splat(b'0'.wrapping_mul(11));
            ans += calc1(data, buf_target.into());
        }
        return ans;
    }
}

unsafe fn calc2(data: u8x64, _targets: u16x32) -> u64 {
    let zeros = i64x16::splat(0);
    let bignum = i64x16::splat(i64::MAX);
    let targets = _targets.cast::<i64>() + i64x32::splat(10000000000000);
    let x1 = data.resize::<16>(0).cast::<i64>();
    let y1 = data.rotate_elements_left::<16>().resize::<16>(0).cast::<i64>();
    let x2 = data.rotate_elements_left::<32>().resize::<16>(0).cast::<i64>();
    let y2 = data.rotate_elements_left::<48>().resize::<16>(0).cast::<i64>();
    let X = targets.resize::<16>(0).cast::<i64>();
    let Y = targets.rotate_elements_left::<16>().resize::<16>(0).cast::<i64>();

    let det = x1 * y2 - x2 * y1;
    let sol1 = det.simd_ne(zeros);
    let det_clean = simd_select(sol1.to_int(), det, bignum);
    // unique solution
    let solA = X * y2 - Y * x2;
    let solB = Y * x1 - X * y1;

    let A_int = solA / det_clean;
    let B_int = solB / det_clean;
    let sol1_valid = ((A_int * det).simd_eq(solA) & (B_int * det).simd_eq(solB)).to_int();

    let A1 = simd_select(sol1_valid, A_int, zeros);
    let B1 = simd_select(sol1_valid, B_int, zeros);

    let mut ans = (A1 * i64x16::splat(3) + B1).reduce_sum() as u64;

    if sol1.all() {
        return ans;
    }
    // no solution
    let sol0 = (x1 * Y - y1 * X).simd_ne(zeros);

    // infinite solutions (collinear)
    let mut solinf = (!sol0 & !sol1).to_bitmask();
    if solinf == 0 { return ans; }
    else {
        // disaster.
        while solinf != 0 {
            let idx = solinf.trailing_zeros() as usize;
            let x1_ = x1[idx];
            let x2_ = x2[idx];
            let X_ = X[idx];
            
            let gcd_ = GCD[(x1_ * 100 + x2_) as usize];
            let gcd = gcd_.gcd as i64;
            let div_ = X_ / gcd;
            let partXA = gcd_.A as i64 * div_;
            let partXB = gcd_.B as i64 * div_;
            let sclA = x2_ / gcd;
            let sclB = x1_ / gcd;
            let k = if x1_ >= x2_ * 3 { (partXB - partXB.rem_euclid(sclB)) / sclB } // Maximize A
                else { -(partXA - partXA.rem_euclid(sclA)) / sclA }; // Maximize B
            let A_final = partXA + k * sclA;
            let B_final = partXB - k * sclB;
            ans += (A_final * 3 + B_final) as u64;
            solinf &= solinf - 1;
        }
    }
    return ans;
}

pub fn part2(s: &str) -> u64 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut ptr = 0;
        let mut buf = [0u8; 128];
        let mut buf_target = [0u16; 32];
        let mut ans = 0u64;
        let _load_idxs = usizex8::from_array([0, 1, 6, 7, 21, 22, 27, 28]);
        let _buf_idxs = usizex8::from_array([0,64,16,80,32,96,48,112]);
        while ptr < bl {
            for buf_ptr in 0..16 {
                ptr += 12;
                let stuff = u8x8::gather_select_unchecked(&b[ptr..ptr + 32], masksizex8::splat(true), _load_idxs, u8x8::splat(0));
                stuff.scatter_select_unchecked(&mut buf[buf_ptr..], masksizex8::splat(true), _buf_idxs);
                ptr += 39;
                let (num1, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
                ptr += len + 4;
                let (num2, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
                ptr += len + 2;
                buf_target[buf_ptr] = num1;
                buf_target[16 + buf_ptr] = num2;
            }
            let word1 = u8x64::load_select_unchecked(&buf[0 ..64 ], mask8x64::splat(true), u8x64::splat(0));
            let word2 = u8x64::load_select_unchecked(&buf[64..128], mask8x64::splat(true), u8x64::splat(0));
            let data = word1 * u8x64::splat(10) + word2 - u8x64::splat(b'0'.wrapping_mul(11));
            ans += calc2(data, buf_target.into());
        }
        return ans;
    }
}
