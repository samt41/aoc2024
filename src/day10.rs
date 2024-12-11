const W: usize = 40;
const WW: usize = W as usize + 1;
const D: [isize; 4] = [-(WW as isize), 1, WW as isize, -1];

pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let mut stack = [0isize; 10];
        let mut dirs = [0usize; 10];
        let mut dests = vec![0u16];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |ans, i| {
            dests.clear();
            let mut depth = 9;
            stack[9] = i as isize;
            while depth <= 9 {
                let dp = dirs[depth];
                if dp == 4 {
                    dirs[depth] = 0;
                    depth += 1;
                    continue;
                }
                let next = stack[depth] + D[dp];
                dirs[depth] = dp + 1;
                if next >= 0 && next < (WW * W) as isize && b[next as usize] == depth as u8 + b'0' - 1 {
                    if depth == 1 {
                        dests.push(next as u16);
                    } else {
                        depth -= 1;
                        stack[depth] = next;
                    }
                }
            }
            dests.sort_unstable();
            dests.dedup();
            return ans + dests.len() as u32;
        });
    }
}

pub fn part2(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let mut stack = [0isize; 10];
        let mut dirs = [0usize; 10];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |ans, i| {
            let mut score = 0u32;
            let mut depth = 9;
            stack[9] = i as isize;
            while depth <= 9 {
                let dp = dirs[depth];
                if dp == 4 {
                    dirs[depth] = 0;
                    depth += 1;
                    continue;
                }
                let next = stack[depth] + D[dp];
                dirs[depth] = dp + 1;
                if next >= 0 && next < (WW * W) as isize && b[next as usize] == depth as u8 + b'0' - 1 {
                    if depth == 1 {
                        score += 1;
                    } else {
                        depth -= 1;
                        stack[depth] = next;
                    }
                }
            }
            return ans + score;
        });
    }
}
