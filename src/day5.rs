pub fn part1(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();

    let mut masks = [0 as u128; 100];
    let mut pos = 0;
    {
        while pos + 6 < bl {
            if b[pos] == b'\n' { break; }
            masks[((b[pos + 3] - b'0') * 10 + b[pos + 4] - b'0') as usize] |=
                (1 as u128) << ((b[pos] - b'0') * 10 + b[pos + 1] - b'0');
            pos += 6;
        }
    }
    pos += 1;
    return s[pos..b.len()].split('\n').fold(0 as u32,|ans, v| {
        let b = v.as_bytes();
        let bl = b.len();
        let mut mask: u128 = 0;
        for i in 0..(bl + 1) / 3 {
            let idx = bl - 2 - i * 3;
            let page = (b[idx] - b'0') * 10 + b[idx + 1] - b'0';
            if masks[page as usize] & mask != 0 { return ans }
            mask |= (1 as u128) << page;
        }
        return ans + ((b[bl / 2 - 1] - b'0') * 10 + b[bl / 2] - b'0') as u32
    });
}

pub fn part2(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();

    let mut masks = [0 as u128; 100];
    let mut pos = 0;
    {
        while pos + 6 < bl {
            if b[pos] == b'\n' { break; }
            masks[((b[pos + 3] - b'0') * 10 + b[pos + 4] - b'0') as usize] |=
                (1 as u128) << ((b[pos] - b'0') * 10 + b[pos + 1] - b'0');
            pos += 6;
        }
    }
    pos += 1;
    let mut buf = vec![0 as u8; 20];
    return s[pos..b.len()].split('\n').fold(0 as u32,|ans, v| {
        let b = v.as_bytes();
        let bl = b.len();
        let num_items = (bl + 1) / 3;
        let mut mask: u128 = 0;
        buf.clear();
        let mut valid = true;
        for i in 0..num_items {
            let idx = bl - 2 - i * 3;
            let page = (b[idx] - b'0') * 10 + b[idx + 1] - b'0';
            buf.push(page);
            valid &= masks[page as usize] & mask == 0;
            mask |= 1 << page;
        }
        if valid { return ans; }
        let mut n = num_items;
        while n > 1 {
            mask = 1 << buf[0];
            let mut new_n = 0;
            for j in 1..n {
                let it = buf[j] as usize;
                let should_swap = masks[it] & mask != 0;
                if should_swap {
                    let tmp = buf[j];
                    buf[j] = buf[j - 1];
                    buf[j - 1] = tmp;
                    mask |= 1 << tmp;
                    new_n = j;
                } else { 
                    mask |= 1 << it;
                }
            }
            n = new_n;
        }
        return ans + buf[num_items / 2] as u32;
    });
}
