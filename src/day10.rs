pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let W = (bl as f32).sqrt() as usize;
        let D = [(!(W+1))+1, 1, W+1, (!1)+1];
        let mut dests = vec![0u16];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |ans, i| {
            dests.clear();
            for dir9 in 0..4 {
                let loc8 = i.wrapping_add(*D.get_unchecked(dir9));
                if loc8 >= bl || *b.get_unchecked(loc8) != b'8' { continue; }
                for dir8 in 0..4 {
                    let loc7 = loc8.wrapping_add(*D.get_unchecked(dir8));
                    if loc7 >= bl || *b.get_unchecked(loc7) != b'7' { continue; }
                    for dir7 in 0..4 {
                        let loc6 = loc7.wrapping_add(*D.get_unchecked(dir7));
                        if loc6 >= bl || *b.get_unchecked(loc6) != b'6' { continue; }
                        for dir6 in 0..4 {
                            let loc5 = loc6.wrapping_add(*D.get_unchecked(dir6));
                            if loc5 >= bl || *b.get_unchecked(loc5) != b'5' { continue; }
                            for dir5 in 0..4 {
                                let loc4 = loc5.wrapping_add(*D.get_unchecked(dir5));
                                if loc4 >= bl || *b.get_unchecked(loc4) != b'4' { continue; }
                                for dir4 in 0..4 {
                                    let loc3 = loc4.wrapping_add(*D.get_unchecked(dir4));
                                    if loc3 >= bl || *b.get_unchecked(loc3) != b'3' { continue; }
                                    for dir3 in 0..4 {
                                        let loc2 = loc3.wrapping_add(*D.get_unchecked(dir3));
                                        if loc2 >= bl || *b.get_unchecked(loc2) != b'2' { continue; }
                                        for dir2 in 0..4 {
                                            let loc1 = loc2.wrapping_add(*D.get_unchecked(dir2));
                                            if loc1 >= bl || *b.get_unchecked(loc1) != b'1' { continue; }
                                            for dir1 in 0..4 {
                                                let loc0 = loc1.wrapping_add(*D.get_unchecked(dir1));
                                                if loc0 < bl && b[loc0] == b'0' {
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
        let W = (bl as f32).sqrt() as usize;
        let D = [(!(W+1))+1, 1, W+1, (!1)+1];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |mut ans, i| {
            for dir9 in 0..4 {
                let loc8 = i.wrapping_add(*D.get_unchecked(dir9));
                if loc8 >= bl || *b.get_unchecked(loc8) != b'8' { continue; }
                for dir8 in 0..4 {
                    let loc7 = loc8.wrapping_add(*D.get_unchecked(dir8));
                    if loc7 >= bl || *b.get_unchecked(loc7) != b'7' { continue; }
                    for dir7 in 0..4 {
                        let loc6 = loc7.wrapping_add(*D.get_unchecked(dir7));
                        if loc6 >= bl || *b.get_unchecked(loc6) != b'6' { continue; }
                        for dir6 in 0..4 {
                            let loc5 = loc6.wrapping_add(*D.get_unchecked(dir6));
                            if loc5 >= bl || *b.get_unchecked(loc5) != b'5' { continue; }
                            for dir5 in 0..4 {
                                let loc4 = loc5.wrapping_add(*D.get_unchecked(dir5));
                                if loc4 >= bl || *b.get_unchecked(loc4) != b'4' { continue; }
                                for dir4 in 0..4 {
                                    let loc3 = loc4.wrapping_add(*D.get_unchecked(dir4));
                                    if loc3 >= bl || *b.get_unchecked(loc3) != b'3' { continue; }
                                    for dir3 in 0..4 {
                                        let loc2 = loc3.wrapping_add(*D.get_unchecked(dir3));
                                        if loc2 >= bl || *b.get_unchecked(loc2) != b'2' { continue; }
                                        for dir2 in 0..4 {
                                            let loc1 = loc2.wrapping_add(*D.get_unchecked(dir2));
                                            if loc1 >= bl || *b.get_unchecked(loc1) != b'1' { continue; }
                                            for dir1 in 0..4 {
                                                let loc0 = loc1.wrapping_add(*D.get_unchecked(dir1));
                                                ans += (loc0 < bl && *b.get_unchecked(loc0) == b'0') as u32;
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
