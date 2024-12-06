pub fn part1(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();

    let mut masks = [0 as u128; 100];
    let mut pos = 0;
    {
        while pos + 6 < bl {
            if b[pos] == b'\n' { break; }
            masks[((b[pos] - b'0') * 10 + b[pos + 1] - b'0') as usize] |=
                (1 as u128) << ((b[pos + 3] - b'0') * 10 + b[pos + 4] - b'0');
            pos += 6;
        }
    }
    pos += 1;
    let mut ans = 0;
    {
        let mut mask: u128 = 0u128;
        let mut valid: bool = true;
        let mut prev_pos: usize = pos;
        while pos + 2 < bl {
            if valid {
                let page: u8 = (b[pos] - b'0') * 10 + b[pos + 1] - b'0';
                valid &= masks[page as usize] & mask == 0;
                mask |= 1 << page;
            }
            if b[pos + 2] == b'\n' {
                mask = 0;
                if valid {
                    ans += ((b[prev_pos + (pos + 2 - prev_pos) / 2 - 1] - b'0') * 10 + b[prev_pos + (pos + 2 - prev_pos) / 2] - b'0') as u32
                }
                valid = true;
                prev_pos = pos + 3;
            }
            pos += 3;
        }
        if valid { // last item
            let page: u8 = (b[pos] - b'0') * 10 + b[pos + 1] - b'0';
            if masks[page as usize] & mask == 0 {
                ans += ((b[prev_pos + (pos + 2 - prev_pos) / 2 - 1] - b'0') * 10 + b[prev_pos + (pos + 2 - prev_pos) / 2] - b'0') as u32
            }
        }
    }
    return ans;
}

pub fn part2(s: &str) -> u32 {
    let b = s.as_bytes();
    let bl = b.len();

    let mut masks = [0 as u128; 256];
    let mut pos = 0;
    {
        while pos + 6 < bl {
            if b[pos] == b'\n' { break; }
            let from = ((b[pos] - b'0') * 10 + b[pos + 1] - b'0') as usize;
            let to = ((b[pos + 3] - b'0') * 10 + b[pos + 4] - b'0') as usize;
            masks[128 + from] |= 1u128 << to;
            masks[to] |= 1u128 << from;
            pos += 6;
        }
    }
    pos += 1;
    let mut ans = 0;
    {
        let mut buf = vec![0 as u8; 20];
        buf.clear();
        let mut mask: u128 = 0u128;
        let mut valid: bool = true;
        let mut prev_pos: usize = pos;
        while pos + 2 <= bl {
            let page: u8 = (b[pos] - b'0') * 10 + b[pos + 1] - b'0';
            buf.push(page);
            valid &= masks[128 + page as usize] & mask == 0;
            mask |= 1 << page;
            if pos + 2 == bl || b[pos + 2] == b'\n' {
                if !valid {
                    let num_items= (pos - prev_pos) / 3 + 1; //(pos + 3 - prev_pos) / 3;
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
                    ans += buf[num_items / 2] as u32;
                }
                mask = 0;
                valid = true;
                prev_pos = pos + 3;
                buf.clear();
            }
            pos += 3;
        }
    }
    return ans;
}
