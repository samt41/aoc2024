use std::mem::MaybeUninit;

use intmap::{ IntMap, Entry };

static mut init: bool = false;
static mut map1: MaybeUninit<IntMap<u64, u64>> = MaybeUninit::uninit();
static mut map2: MaybeUninit<IntMap<u64, u64>> = MaybeUninit::uninit();
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

unsafe fn _loop(iters: u32) -> u64 {
    let m1 = map1.assume_init_mut();
    let m2 = map2.assume_init_mut();
    for _ in 0..iters/2 {
        m2.clear();
        let zero = m1.remove(0);
        if zero.is_some() {
            m2.insert(1, zero.unwrap_unchecked());
        }
        m1.drain().for_each(|(k, v)| {
            let lg = k.ilog10();
            if lg & 1 == 1 {
                let md = 10u64.pow((lg + 1) >> 1);
                let (x, y) = (k / md, k % md);
                *match m2.entry(x) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(0),
                } += v;
                *match m2.entry(y) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(0),
                } += v;
            } else {
                *match m2.entry(k * 2024) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(0),
                } += v;
            }
        });

        m1.clear();
        let zero = m2.remove(0);
        if zero.is_some() {
            m1.insert(1, zero.unwrap_unchecked());
        }
        m2.drain().for_each(|(k, v)| {
            let lg = k.ilog10();
            if lg & 1 == 1 {
                let md = 10u64.pow((lg + 1) >> 1);
                let (x, y) = (k / md, k % md);
                *match m1.entry(x) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(0),
                } += v;
                *match m1.entry(y) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(0),
                } += v;
            } else {
                *match m1.entry(k * 2024) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(0),
                } += v;
            }
        });
    }
    let ans = m1.remove(0).unwrap_or(0) + 
    m1.drain().fold(0u64, |i, (k, v)| {
        let lg = k.ilog10();
        return i + (v << (lg & 1));
    });
    return ans;
}

pub fn part1(s: &str) -> u32 {
    unsafe {
        if !init { _init1() }
        let m = map1.assume_init_mut();
        m.clear();
        let b = s.as_bytes();
        let mut i = 0;
        loop {
            let res = atoi_simd::parse_any_pos::<u64>(&b[i..]);
            match res {
                Ok((v, l)) => {
                    *match m.entry(v) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += 1;
                    i += l + 1;
                },
                Err(_) => return _loop(25) as u32
            }
        }
    }
}

pub fn part2(s: &str) -> u64 {
    unsafe {
        if !init { _init2() }
        let m = map1.assume_init_mut();
        m.clear();
        let b = s.as_bytes();
        let mut i = 0;
        loop {
            let res = atoi_simd::parse_any_pos::<u64>(&b[i..]);
            match res {
                Ok((v, l)) => {
                    *match m.entry(v) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(0),
                    } += 1;
                    i += l + 1;
                },
                Err(_) => return _loop(75)
            }
        }
    }
}
