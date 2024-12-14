use core::simd::prelude::*;
use std::intrinsics::simd::simd_select;

#[derive(Clone, Copy)]
struct GcdObj {
    gcd: u8,
    A: u8,
    B: u8,
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
    *y = x1 - y1 * (a / b);
    return d;
}

static GCD: [GcdObj; 10000] = {
    let mut arr = [GcdObj{gcd: 0, A: 0, B: 0, _pad: 0}; 10000];
    let mut i = 0;
    while i < 10000 {
        let mut x = 0i8;
        let mut y = 0i8;
        arr[i].gcd = build_gcd((i as u8 / 100) as i8, (i as u8 % 100) as i8, &mut x, &mut y) as u8;
        arr[i].A = x as u8;
        arr[i].B = y as u8;
        i += 1;
    }
    arr
};

// there are 320 test cases.
// each test case needs 4 input numbers. so we can do them in chunks of 16
unsafe fn calc1(data: u8x64, targets: u16x32) -> u32 {
    let x1 = simd_swizzle!(data, [0, 4,  8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60]).cast::<i32>();
    let y1 = simd_swizzle!(data, [1, 5,  9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61]).cast::<i32>();
    let x2 = simd_swizzle!(data, [2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 62]).cast::<i32>();
    let y2 = simd_swizzle!(data, [3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63]).cast::<i32>();
    let X = simd_swizzle!(targets, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]).cast::<i32>();
    let Y = simd_swizzle!(targets, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]).cast::<i32>();
    let det = x1 * y2 - x2 * y1;
    let sol1 = det.simd_ne(i32x16::splat(0));
    // unique solution
    let solA = X * y2 - Y * x2;
    let solB = Y * x1 - X * y1;
    // can we just unsafe divide by 0? H
    let A_int = solA / simd_select(sol1.to_int(), det, i32x16::splat(1));
    let B_int = solB / simd_select(sol1.to_int(), det, i32x16::splat(1));
    //let sol1_is_integer = (solA % simd_select(sol1.to_int(), det, u32x16::splat(1))).simd_eq(u32x16::splat(0)) &
    //    (solB % simd_select(sol1.to_int(), det, u32x16::splat(1))).simd_eq(u32x16::splat(0));
    let sol1_is_integer = (A_int * det).simd_eq(solA) & (B_int * det).simd_eq(solB);
    let sol1_in_range = A_int.cast::<u32>().simd_le(u32x16::splat(100)) & B_int.cast::<u32>().simd_le(u32x16::splat(100));
    let sol1_valid = (sol1_is_integer & sol1_in_range).to_int();

    let A1 = simd_select(sol1_valid, A_int, i32x16::splat(0));
    let B1 = simd_select(sol1_valid, B_int, i32x16::splat(0));

    let mut ans = (A1 * i32x16::splat(3) + B1).reduce_sum() as u32;

    if sol1.all() {
        return ans;
    }
    // no solution
    let sol0 = (x1 * Y - y1 * X).simd_ne(i32x16::splat(0));
    // infinite solutions (collinear)
    let mut solinf = (!sol0 & !sol1).to_bitmask();
    if solinf == 0 { return ans; }
    else {
        // disaster.
        while solinf != 0 {
            let idx = solinf.trailing_zeros() as usize;
            // println!("Trying {idx}");
            let x1_ = x1[idx];
            let x2_ = x2[idx];
            let X_ = X[idx];
            
            let gcd_ = GCD[(x1_ * 100 + x2_) as usize];
            let gcd = gcd_.gcd as i32;
            // println!("x1: {x1_}, x2: {x2_}, y1: {}, y2: {}, X:{X_}, Y:{}", y1[idx], y2[idx], Y[idx]);
            let div_ = X_ / gcd;
            // let mod_ = X_ % gcd;
            // if mod_ == 0, panic and cry.
            let partXA = gcd_.A as i32 * div_;
            let partXB = gcd_.B as i32 * div_;
            let sclA = x2_ / gcd;
            let sclB = x1_ / gcd;
            let k: i32 = if x1_ >= x2_ * 3 { (partXB / sclB) as i32 } // Maximize A
                else { -((partXA / sclA) as i32) }; // Maximize B
            let A_final = partXA as i32 + k * sclA as i32;
            let B_final = partXB as i32 - k * sclB as i32;
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
        let mut buf = [0u8; 64];
        let mut buf_target = [0u16; 32];
        let mut buf_ptr = 0;
        let mut buf_target_ptr = 0;
        let mut ans = 0u32;
        while ptr < bl {
            ptr += 12;
            let stuff= u8x32::load_select_unchecked(&b[ptr..ptr + 32], mask8x32::splat(true), u8x32::splat(0));
            let stuff2 = simd_swizzle!(stuff, [0, 1, 6, 7, 21, 22, 27, 28]) - u8x8::splat(b'0');
            let stuff3 = simd_swizzle!(stuff2, [0, 2, 4, 6]) * u8x4::splat(10);
            let stuff4 = stuff3 + simd_swizzle!(stuff2, [1, 3, 5, 7]);
            stuff4.store_select_unchecked(&mut buf[buf_ptr..buf_ptr+4], mask8x4::splat(true));
            ptr += 39;
            let (num1, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
            ptr += len + 4;
            let (num2, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
            ptr += len + 2;
            buf_target[buf_target_ptr] = num1;
            buf_target[buf_target_ptr + 1] = num2;
            if buf_ptr == 60 {
                ans += calc1(buf.into(), buf_target.into());
            }
            buf_ptr = (buf_ptr + 4) & 63;
            buf_target_ptr = (buf_target_ptr + 2) & 31;
        }
        return ans;
    }
}

unsafe fn calc2(data: u8x64, _targets: u16x32) -> u64 {
    let targets = _targets.cast::<i64>() + i64x32::splat(10000000000000);
    let x1 = simd_swizzle!(data, [0, 4,  8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60]).cast::<i64>();
    let y1 = simd_swizzle!(data, [1, 5,  9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61]).cast::<i64>();
    let x2 = simd_swizzle!(data, [2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 62]).cast::<i64>();
    let y2 = simd_swizzle!(data, [3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63]).cast::<i64>();
    let X = simd_swizzle!(targets, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]);
    let Y = simd_swizzle!(targets, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]);
    let det = x1 * y2 - x2 * y1;
    let sol1 = det.simd_ne(i64x16::splat(0));
    // unique solution
    let solA = X * y2 - Y * x2;
    let solB = Y * x1 - X * y1;

    let A_int = solA / simd_select(sol1.to_int(), det, i64x16::splat(1));
    let B_int = solB / simd_select(sol1.to_int(), det, i64x16::splat(1));
    let sol1_valid = ((A_int * det).simd_eq(solA) & (B_int * det).simd_eq(solB)).to_int();

    let A1 = simd_select(sol1_valid, A_int, i64x16::splat(0));
    let B1 = simd_select(sol1_valid, B_int, i64x16::splat(0));

    let mut ans = (A1 * i64x16::splat(3) + B1).reduce_sum() as u64;

    if sol1.all() {
        return ans;
    }
    // no solution
    let sol0 = (x1 * Y - y1 * X).simd_ne(i64x16::splat(0));
    // infinite solutions (collinear)
    let mut solinf = (!sol0 & !sol1).to_bitmask();
    if solinf == 0 { return ans; }
    else {
        // disaster.
        while solinf != 0 {
            let idx = solinf.trailing_zeros() as usize;
            // println!("Trying {idx}");
            let x1_ = x1[idx];
            let x2_ = x2[idx];
            let X_ = X[idx];
            
            let gcd_ = GCD[(x1_ * 100 + x2_) as usize];
            let gcd = gcd_.gcd as i64;
            // println!("x1: {x1_}, x2: {x2_}, y1: {}, y2: {}, X:{X_}, Y:{}", y1[idx], y2[idx], Y[idx]);
            let div_ = X_ / gcd;
            // let mod_ = X_ % gcd;
            // if mod_ == 0, panic and cry.
            let partXA = gcd_.A as i64 * div_;
            let partXB = gcd_.B as i64 * div_;
            let sclA = x2_ / gcd;
            let sclB = x1_ / gcd;
            let k: i64 = if x1_ >= x2_ * 3 { (partXB / sclB) as i64 } // Maximize A
                else { -((partXA / sclA) as i64) }; // Maximize B
            let A_final = partXA as i64 + k * sclA as i64;
            let B_final = partXB as i64 - k * sclB as i64;
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
        let mut buf = [0u8; 64];
        let mut buf_target = [0u16; 32];
        let mut buf_ptr = 0;
        let mut buf_target_ptr = 0;
        let mut ans = 0u64;
        while ptr < bl {
            ptr += 12;
            let stuff= u8x32::load_select_unchecked(&b[ptr..ptr + 32], mask8x32::splat(true), u8x32::splat(0));
            let stuff2 = simd_swizzle!(stuff, [0, 1, 6, 7, 21, 22, 27, 28]) - u8x8::splat(b'0');
            let stuff3 = simd_swizzle!(stuff2, [0, 2, 4, 6]) * u8x4::splat(10);
            let stuff4 = stuff3 + simd_swizzle!(stuff2, [1, 3, 5, 7]);
            stuff4.store_select_unchecked(&mut buf[buf_ptr..buf_ptr+4], mask8x4::splat(true));
            ptr += 39;
            let (num1, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
            ptr += len + 4;
            let (num2, len) = atoi_simd::parse_any_pos::<u16>(&b[ptr..]).unwrap_unchecked();
            ptr += len + 2;
            buf_target[buf_target_ptr] = num1;
            buf_target[buf_target_ptr + 1] = num2;
            if buf_ptr == 60 {
                ans += calc2(buf.into(), buf_target.into());
            }
            buf_ptr = (buf_ptr + 4) & 63;
            buf_target_ptr = (buf_target_ptr + 2) & 31;
        }
        return ans;
    }
}
