pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let W = (bl as f32).sqrt() as isize;
        let D = [-W-1, 1, W+1, -1];
        let mut dests = vec![0u16];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |ans, i| {
            dests.clear();
            for dir9 in 0..4 {
                let loc8 = i as isize + D[dir9];
                if loc8 < 0 || loc8 as usize >= bl || b[loc8 as usize] != b'8' { continue; }
                for dir8 in 0..4 {
                    let loc7 = loc8 + D[dir8];
                    if loc7 < 0 || loc7 as usize >= bl || b[loc7 as usize] != b'7' { continue; }
                    for dir7 in 0..4 {
                        let loc6 = loc7 + D[dir7];
                        if loc6 < 0 || loc6 as usize >= bl || b[loc6 as usize] != b'6' { continue; }
                        for dir6 in 0..4 {
                            let loc5 = loc6 + D[dir6];
                            if loc5 < 0 || loc5 as usize >= bl || b[loc5 as usize] != b'5' { continue; }
                            for dir5 in 0..4 {
                                let loc4 = loc5 + D[dir5];
                                if loc4 < 0 || loc4 as usize >= bl || b[loc4 as usize] != b'4' { continue; }
                                for dir4 in 0..4 {
                                    let loc3 = loc4 + D[dir4];
                                    if loc3 < 0 || loc3 as usize >= bl || b[loc3 as usize] != b'3' { continue; }
                                    for dir3 in 0..4 {
                                        let loc2 = loc3 + D[dir3];
                                        if loc2 < 0 || loc2 as usize >= bl || b[loc2 as usize] != b'2' { continue; }
                                        for dir2 in 0..4 {
                                            let loc1 = loc2 + D[dir2];
                                            if loc1 < 0 || loc1 as usize >= bl || b[loc1 as usize] != b'1' { continue; }
                                            for dir1 in 0..4 {
                                                let loc0 = loc1 + D[dir1];
                                                if loc0 >= 0 && (loc0 as usize) < bl && b[loc0 as usize] == b'0' {
                                                    dests.push(loc0 as u16);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
        let bl = b.len();
        let W = (bl as f32).sqrt() as isize;
        let D = [-W-1, 1, W+1, -1];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |mut ans, i| {
            for dir9 in 0..4 {
                let loc8 = i as isize + D[dir9];
                if loc8 < 0 || loc8 as usize >= bl || b[loc8 as usize] != b'8' { continue; }
                for dir8 in 0..4 {
                    let loc7 = loc8 + D[dir8];
                    if loc7 < 0 || loc7 as usize >= bl || b[loc7 as usize] != b'7' { continue; }
                    for dir7 in 0..4 {
                        let loc6 = loc7 + D[dir7];
                        if loc6 < 0 || loc6 as usize >= bl || b[loc6 as usize] != b'6' { continue; }
                        for dir6 in 0..4 {
                            let loc5 = loc6 + D[dir6];
                            if loc5 < 0 || loc5 as usize >= bl || b[loc5 as usize] != b'5' { continue; }
                            for dir5 in 0..4 {
                                let loc4 = loc5 + D[dir5];
                                if loc4 < 0 || loc4 as usize >= bl || b[loc4 as usize] != b'4' { continue; }
                                for dir4 in 0..4 {
                                    let loc3 = loc4 + D[dir4];
                                    if loc3 < 0 || loc3 as usize >= bl || b[loc3 as usize] != b'3' { continue; }
                                    for dir3 in 0..4 {
                                        let loc2 = loc3 + D[dir3];
                                        if loc2 < 0 || loc2 as usize >= bl || b[loc2 as usize] != b'2' { continue; }
                                        for dir2 in 0..4 {
                                            let loc1 = loc2 + D[dir2];
                                            if loc1 < 0 || loc1 as usize >= bl || b[loc1 as usize] != b'1' { continue; }
                                            for dir1 in 0..4 {
                                                let loc0 = loc1 + D[dir1];
                                                ans += (loc0 >= 0 && (loc0 as usize) < bl && b[loc0 as usize] == b'0') as u32;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return ans as u32;
        });
    }
}
