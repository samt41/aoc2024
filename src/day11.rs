use std::mem::MaybeUninit;
use std::simd::prelude::*;

use intmap::{ IntMap, Entry };

static mut init: bool = false;
static mut map1: MaybeUninit<IntMap<u64, u64>> = MaybeUninit::uninit();
static mut map2: MaybeUninit<IntMap<u64, u64>> = MaybeUninit::uninit();
const L: usize = 960;

unsafe fn _init1() {
    map1.write(IntMap::with_capacity(512));
    map2.write(IntMap::with_capacity(512));
    init = true;
}

unsafe fn _init2() {
    map1.write(IntMap::with_capacity(4096));
    map2.write(IntMap::with_capacity(4096));
    init = true;
}

unsafe fn _loop(iters: u32, mem: &mut [u64; L * 2]) -> u64 {
    let m1 = map1.assume_init_mut();
    let m2 = map2.assume_init_mut();
    for _ in 0..iters/2 {
        m2.clear();
        mem[L..].fill(0);
        mem[L + 1] = mem[0];
        for i in 1..10 {
            if mem[i] > 0 {
                m2.insert(i as u64 * 2024, mem[i]);
            }
        }
        for i in 10..100 {
            let v = mem[i];
            mem[L + i / 10] += v;
            mem[L + i % 10] += v;
        }
        for i in 100..L {
            if mem[i] > 0 {
                m2.insert(i as u64 * 2024, mem[i]);
            }
        }
        m1.drain().for_each(|(k, v)| {
            let lg = k.ilog10();
            if lg & 1 == 1 {
                let md = 10u64.pow((lg + 1) >> 1);
                let (x, y) = (k / md, k % md);
                if x < L as u64 { mem[L + x as usize] += v }
                else {
                    *match m2.entry(x) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += v;
                }
                if y < L as u64 { mem[L + y as usize] += v }
                else {
                    *match m2.entry(y) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += v;
                }
            } else {
                let x = k * 2024;
                if x < L as u64 { mem[L + x as usize] += v }
                else {
                    *match m2.entry(k * 2024) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += v;
                }
            }
        });

        m1.clear();
        mem[..L].fill(0);
        mem[1] = mem[L];
        for i in 1..10 {
            if mem[L + i] > 0 {
                m1.insert(i as u64 * 2024, mem[L + i]);
            }
        }
        for i in 10..100 {
            let v = mem[L + i];
            mem[i / 10] += v;
            mem[i % 10] += v;
        }
        for i in 100..L {
            if mem[L + i] > 0 {
                m1.insert(i as u64 * 2024, mem[L + i]);
            }
        }
        m2.drain().for_each(|(k, v)| {
            let lg = k.ilog10();
            if lg & 1 == 1 {
                let md = 10u64.pow((lg + 1) >> 1);
                let (x, y) = (k / md, k % md);
                if x < L as u64 { mem[x as usize] += v }
                else {
                    *match m1.entry(x) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += v;
                }
                if y < L as u64 { mem[y as usize] += v }
                else {
                    *match m1.entry(y) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += v;
                }
            } else {
                let x = k * 2024;
                if x < L as u64 { mem[x as usize] += v }
                else {
                    *match m1.entry(k * 2024) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += v;
                }
            }
        });
    }
    let mut ans = 0u64;
    for i in (0..L).step_by(64) {
        ans += u64x64::load_select_unchecked(&mem[i..i+64], mask64x64::splat(true), u64x64::splat(0)).reduce_sum();
    }
    ans += u64x64::load_select_unchecked(&mem[10..74], mask64x64::splat(true), u64x64::splat(0)).reduce_sum();
    ans += u64x32::load_select_unchecked(&mem[74..100], mask64x32::from_bitmask((1 << 26) - 1), u64x32::splat(0)).reduce_sum();
    return m1.drain().fold(ans, |i, (k, v)| {
        let lg = k.ilog10();
        return i + (v << (lg & 1));
    });
}

pub fn part1(s: &str) -> u32 {
    unsafe {
        let scratch = &mut [0u64; L * 2];
        if !init { _init1() }
        let m = map1.assume_init_mut();
        m.clear();
        let b = s.as_bytes();
        let mut i = 0;
        loop {
            let res = atoi_simd::parse_any_pos::<u64>(&b[i..]);
            match res {
                Ok((v, l)) => {
                    if v < L as u64 { scratch[v as usize] += 1; }
                    else {
                        *match m.entry(v) {
                            Entry::Occupied(entry) => entry.into_mut(),
                            Entry::Vacant(entry) => entry.insert(0),
                        } += 1;
                    }
                    i += l + 1;
                },
                Err(_) => return _loop(25, scratch) as u32
            }
        }
    }
}

pub fn part2(s: &str) -> u64 {
    unsafe {
        if !init { _init2() }
        let scratch = &mut [0u64; L * 2];
        let m = map1.assume_init_mut();
        m.clear();
        let b = s.as_bytes();
        let mut i = 0;
        loop {
            let res = atoi_simd::parse_any_pos::<u64>(&b[i..]);
            match res {
                Ok((v, l)) => {
                    if v < L as u64 { scratch[v as usize] += 1; }
                    else {
                        *match m.entry(v) {
                            Entry::Occupied(entry) => entry.into_mut(),
                            Entry::Vacant(entry) => entry.insert(0),
                        } += 1;
                    }
                    i += l + 1;
                },
                Err(_) => return _loop(75, scratch)
            }
        }
    }
}
