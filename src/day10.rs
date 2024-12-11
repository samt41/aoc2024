pub fn part1(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let w = (bl as f32).sqrt() as usize;
        let d = [(!(w+1))+1, 1, w+1, (!1)+1];
        let mut vis = [0u64; 128];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |mut ans, i| {
            for dir9 in 0..4 {
                let loc8 = i.wrapping_add(*d.get_unchecked(dir9));
                if loc8 < bl && *b.get_unchecked(loc8) == b'8' {
                    for dir8 in 0..4 {
                        let loc7 = loc8.wrapping_add(*d.get_unchecked(dir8));
                        if loc7 < bl && *b.get_unchecked(loc7) == b'7' {
                            for dir7 in 0..4 {
                                let loc6 = loc7.wrapping_add(*d.get_unchecked(dir7));
                                if loc6 < bl && *b.get_unchecked(loc6) == b'6' {
                                    for dir6 in 0..4 {
                                        let loc5 = loc6.wrapping_add(*d.get_unchecked(dir6));
                                        if loc5 < bl && *b.get_unchecked(loc5) == b'5' {
                                            for dir5 in 0..4 {
                                                let loc4 = loc5.wrapping_add(*d.get_unchecked(dir5));
                                                if loc4 < bl && *b.get_unchecked(loc4) == b'4' {
                                                    for dir4 in 0..4 {
                                                        let loc3 = loc4.wrapping_add(*d.get_unchecked(dir4));
                                                        if loc3 < bl && *b.get_unchecked(loc3) == b'3' {
                                                            for dir3 in 0..4 {
                                                                let loc2 = loc3.wrapping_add(*d.get_unchecked(dir3));
                                                                if loc2 < bl && *b.get_unchecked(loc2) == b'2' {
                                                                    for dir2 in 0..4 {
                                                                        let loc1 = loc2.wrapping_add(*d.get_unchecked(dir2));
                                                                        if loc1 < bl && *b.get_unchecked(loc1) == b'1' {
                                                                            for dir1 in 0..4 {
                                                                                let loc0 = loc1.wrapping_add(*d.get_unchecked(dir1));
                                                                                if loc0 < bl && *b.get_unchecked(loc0) == b'0' {
                                                                                    // let prev = vis[loc0 >> 6];
                                                                                    vis[loc0 >> 6] |= 1 << (loc0 & 63);
                                                                                    // ans += (vis[loc0 >> 6] - prev).count_ones();
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for j in 0..80 {
                ans += vis[j].count_ones();
                vis[j] = 0;
            }
            return ans;
        });
    }
}

pub fn part2(s: &str) -> u32 {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let w = (bl as f32).sqrt() as usize;
        let d = [(!(w+1))+1, 1, w+1, (!1)+1];
        let points = memchr::memchr_iter(b'9', b);
        return points.fold(0u32, |mut ans, i| {
            for dir9 in 0..4 {
                let loc8 = i.wrapping_add(*d.get_unchecked(dir9));
                if loc8 < bl && *b.get_unchecked(loc8) == b'8' {
                    for dir8 in 0..4 {
                        let loc7 = loc8.wrapping_add(*d.get_unchecked(dir8));
                        if loc7 < bl && *b.get_unchecked(loc7) == b'7' {
                            for dir7 in 0..4 {
                                let loc6 = loc7.wrapping_add(*d.get_unchecked(dir7));
                                if loc6 < bl && *b.get_unchecked(loc6) == b'6' {
                                    for dir6 in 0..4 {
                                        let loc5 = loc6.wrapping_add(*d.get_unchecked(dir6));
                                        if loc5 < bl && *b.get_unchecked(loc5) == b'5' {
                                            for dir5 in 0..4 {
                                                let loc4 = loc5.wrapping_add(*d.get_unchecked(dir5));
                                                if loc4 < bl && *b.get_unchecked(loc4) == b'4' {
                                                    for dir4 in 0..4 {
                                                        let loc3 = loc4.wrapping_add(*d.get_unchecked(dir4));
                                                        if loc3 < bl && *b.get_unchecked(loc3) == b'3' {
                                                            for dir3 in 0..4 {
                                                                let loc2 = loc3.wrapping_add(*d.get_unchecked(dir3));
                                                                if loc2 < bl && *b.get_unchecked(loc2) == b'2' {
                                                                    for dir2 in 0..4 {
                                                                        let loc1 = loc2.wrapping_add(*d.get_unchecked(dir2));
                                                                        if loc1 < bl && *b.get_unchecked(loc1) == b'1' {
                                                                            for dir1 in 0..4 {
                                                                                let loc0 = loc1.wrapping_add(*d.get_unchecked(dir1));
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
