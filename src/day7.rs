unsafe fn permute1(num: u64, idx: usize, st: &[u32; 16]) -> bool {
    let next = st[idx] as u64;
    let div = num / next;
    let rem = num % next;

    return
        idx == 0 && num == next ||
        idx > 0 && (
            (rem == 0 && permute1(div, idx - 1, &st)) ||
            (num >= next && permute1(num - next, idx - 1, &st))
        );
}

pub fn part1(s: &str) -> u64 {
    unsafe {
        let mut st: [u32; 16] = [0u32; 16];
        let b = s.as_bytes();
        let bl = b.len();
        let mut ans = 0u64;

        let parsed = atoi_simd::parse_any_pos::<u64>(&b[0..16]).unwrap_unchecked();
        let mut target = parsed.0;
        let mut curr = 0u32;
        let mut i: usize = 0;
        let mut x = parsed.1 + 2;
        while x < bl {
            match b[x] {
                b'\n' => {
                    st[i] = curr;
                    if permute1(target, i, &st) { ans += target; }
                    if x + 1 >= bl { return ans; }
                    let parsed = atoi_simd::parse_any_pos::<u64>(&b[x+1..(x+17).min(bl)]).unwrap_unchecked();
                    target = parsed.0;
                    x += parsed.1 + 2;
                    curr = 0;
                    i = 0;
                },
                b' ' => {
                    st[i] = curr;
                    i += 1;
                    curr = 0;
                },
                v => {
                    curr = curr * 10 + (v - b'0') as u32;
                }
            }
            x += 1;
        }
        return ans;
    }
}

unsafe fn permute2(num: u64, idx: usize, st: &[u32; 16]) -> bool {
    let next = st[idx] as u64;
    if idx == 0 { return num == next }

    let div = num / next;
    let rem = num % next;
    if (rem == 0 && permute2(div, idx - 1, &st)) ||
           (num >= next && permute2(num - next, idx - 1, &st)) { return true; }
    let log = if next >= u64::pow(10, 6) {
        if next >= u64::pow(10, 10) {
            if next >= u64::pow(10, 12) {
                if next >= u64::pow(10, 13) {
                    if next >= u64::pow(10, 14) { u64::pow(10, 15) }
                    else { u64::pow(10, 14) }
                } else { u64::pow(10, 13) }
            } else {
                if next >= u64::pow(10, 11) { u64::pow(10, 12) }
                else { u64::pow(10, 11) }
            }
        } else {
            if next >= u64::pow(10, 8) {
                if next >= u64::pow(10, 9) { u64::pow(10, 10) }
                else { u64::pow(10, 9) }
            } else {
                if next >= u64::pow(10, 7) { u64::pow(10, 8) }
                else { u64::pow(10, 7) }
            }
        }
    } else {
        if next >= u64::pow(10, 4) {
            if next >= u64::pow(10, 5) { u64::pow(10, 6) }
            else { u64::pow(10, 5) }
        } else {
            if next >= u64::pow(10, 2) {
                if next >= u64::pow(10, 3) { u64::pow(10, 4) } else { u64::pow(10, 3) }
            } else {
                if next >= u64::pow(10, 1) { u64::pow(10, 2) }
                else { u64::pow(10, 1) }
            }
        }
    };
    let div_ = num / log;
    let rem_ = num % log;
    return rem_ == next && permute2(div_, idx - 1, &st);
}

pub fn part2(s: &str) -> u64 {
    unsafe {
        let mut st: [u32; 16] = [0u32; 16];
        let b = s.as_bytes();
        let bl = b.len();
        let mut ans = 0u64;

        let parsed = atoi_simd::parse_any_pos::<u64>(&b[0..16]).unwrap_unchecked();
        let mut target = parsed.0;
        let mut curr = 0u32;
        let mut i: usize = 0;
        let mut x = parsed.1 + 2;
        while x < bl {
            match b[x] {
                b'\n' => {
                    st[i] = curr;
                    if permute2(target, i, &st) { ans += target; }
                    if x + 1 >= bl { return ans; }
                    let parsed = atoi_simd::parse_any_pos::<u64>(&b[x+1..(x+17).min(bl)]).unwrap_unchecked();
                    target = parsed.0;
                    x += parsed.1 + 2;
                    curr = 0;
                    i = 0;
                },
                b' ' => {
                    st[i] = curr;
                    i += 1;
                    curr = 0;
                },
                v => {
                    curr = curr * 10 + (v - b'0') as u32;
                }
            }
            x += 1;
        }
        return ans;
    }
}
