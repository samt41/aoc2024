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
        let mut memo = vec![u16::MAX; bl * 4];
        let w = (bl as f32).sqrt() as usize;
        let d = [(!(w+1))+1, 1, w+1, (!1)+1];
        let points = memchr::memchr_iter(b'9', b);
        let mem = memo.as_mut_ptr();
        return points.fold(0u32, |mut ans, i| {
            for dir9 in 0..4 {
                let loc8 = i.wrapping_add(*d.get_unchecked(dir9));
                if loc8 < bl && *b.get_unchecked(loc8) == b'8' {
                    // let mem8 = mem.add((loc8 << 2) | dir9);
                    // if *mem8 != u16::MAX { ans += *mem8 as u32; continue; }
                    // let ans8 = ans;
                    for dir8 in 0..4 {
                        let loc7 = loc8.wrapping_add(*d.get_unchecked(dir8));
                        if loc7 < bl && *b.get_unchecked(loc7) == b'7' {
                            let mem7 = mem.add((loc7 << 2) | dir8);
                            if *mem7 != u16::MAX { ans += *mem7 as u32; continue; }
                            let ans7 = ans;
                            for dir7 in 0..4 {
                                let loc6 = loc7.wrapping_add(*d.get_unchecked(dir7));
                                if loc6 < bl && *b.get_unchecked(loc6) == b'6' {
                                    // let mem6 = mem.add((loc6 << 2) | dir7);
                                    // if *mem6 != u16::MAX { ans += *mem6 as u32; continue; }
                                    // let ans6 = ans;
                                    for dir6 in 0..4 {
                                        let loc5 = loc6.wrapping_add(*d.get_unchecked(dir6));
                                        if loc5 < bl && *b.get_unchecked(loc5) == b'5' {
                                            let mem5 = mem.add((loc5 << 2) | dir6);
                                            if *mem5 != u16::MAX { ans += *mem5 as u32; continue; }
                                            let ans5 = ans;
                                            for dir5 in 0..4 {
                                                let loc4 = loc5.wrapping_add(*d.get_unchecked(dir5));
                                                if loc4 < bl && *b.get_unchecked(loc4) == b'4' {
                                                    let mem4 = mem.add((loc4 << 2) | dir5);
                                                    if *mem4 != u16::MAX { ans += *mem4 as u32; continue; }
                                                    let ans4 = ans;
                                                    for dir4 in 0..4 {
                                                        let loc3 = loc4.wrapping_add(*d.get_unchecked(dir4));
                                                        if loc3 < bl && *b.get_unchecked(loc3) == b'3' {
                                                            let mem3 = mem.add((loc3 << 2) | dir4);
                                                            if *mem3 != u16::MAX { ans += *mem3 as u32; continue; }
                                                            let ans3 = ans;
                                                            for dir3 in 0..4 {
                                                                let loc2 = loc3.wrapping_add(*d.get_unchecked(dir3));
                                                                if loc2 < bl && *b.get_unchecked(loc2) == b'2' {
                                                                    // let mem2 = mem.add((loc2 << 2) | dir3);
                                                                    // if *mem2 != u16::MAX { ans += *mem2 as u32; continue; }
                                                                    // let ans2 = ans;
                                                                    for dir2 in 0..4 {
                                                                        let loc1 = loc2.wrapping_add(*d.get_unchecked(dir2));
                                                                        if loc1 < bl && *b.get_unchecked(loc1) == b'1' {
                                                                            let mem1 = mem.add((loc1 << 2) | dir2);
                                                                            if *mem1 != u16::MAX { ans += *mem1 as u32; continue; }
                                                                            let ans1 = ans;
                                                                            for dir1 in 0..4 {
                                                                                let loc0 = loc1.wrapping_add(*d.get_unchecked(dir1));
                                                                                ans += (loc0 < bl && *b.get_unchecked(loc0) == b'0') as u32;
                                                                            }
                                                                            *mem1 = (ans - ans1) as u16;
                                                                        }
                                                                    }
                                                                    // *mem2 = (ans - ans2) as u16;
                                                                }
                                                            }
                                                            *mem3 = (ans - ans3) as u16;
                                                        }
                                                    }
                                                    *mem4 = (ans - ans4) as u16;
                                                }
                                            }
                                            *mem5 = (ans - ans5) as u16;
                                        }
                                    }
                                    // *mem6 = (ans - ans6) as u16;
                                }
                            }
                            *mem7 = (ans - ans7) as u16;
                        }
                    }
                    // *mem8 = (ans - ans8) as u16;
                }
            }
            return ans as u32;
        });
    }
}
