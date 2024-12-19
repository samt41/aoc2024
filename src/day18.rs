use std::simd::{prelude::*, ToBytes};

const SHUF: [[[u8; 8]; 2]; 2] = [
    [
        [7, 0, 7, 7, 7, 2, 7, 7],
        [7, 0, 7, 7, 7, 3, 7, 2],
    ],
    [
        [7, 1, 7, 0, 7, 3, 7, 7],
        [7, 1, 7, 0, 7, 4, 7, 3],
    ]
];
const SIZE_REAL: usize = 71;
const SIZE_REAL16: u16 = SIZE_REAL as u16;
const SIZE_LOGICAL: usize = 128;
const SIZE_LOGICAL16: u16 = SIZE_LOGICAL as u16;

pub fn part1(s: &str) -> u16 {
    unsafe {
        let b = s.as_bytes();
        let mut grid = [0u8; (SIZE_LOGICAL+1)*(SIZE_REAL+1)];
        let mut sep = memchr::memchr2_iter(b'\n', b',', b);
        {
            let mut last_pos = -1;
            let mut num_lines = 0;
            while num_lines < 1024 {
                let pos1 = sep.next().unwrap_unchecked() as i32;
                let first_idx = pos1 - last_pos - 2;
                let pos2 = sep.next().unwrap_unchecked() as i32;
                let second_idx = pos2 - pos1 - 2;
                let mut line = u8x8::load_select_unchecked(&b[(last_pos+1) as usize..pos2 as usize],
                    mask8x8::from_bitmask((1 << (pos2 - last_pos - 1)) - 1), u8x8::splat(b'0'));
                line -= u8x8::splat(b'0');
                let mut s = u16x4::from_be_bytes(
                    line.swizzle_dyn(u8x8::from_array(
                        SHUF[first_idx as usize][second_idx as usize]))
                );
                s *= u16x4::from_array([1, 10, SIZE_LOGICAL16, SIZE_LOGICAL16 * 10]);
                grid[s.reduce_sum() as usize] |= 4;
                last_pos = pos2;
                num_lines += 1;
            }
        }
        {
            let mut queue = [0u16; (SIZE_LOGICAL+2)*(SIZE_REAL) - 1024];
            queue[0] = 0;
            queue[1] = ((SIZE_REAL16 - 1) * SIZE_LOGICAL16 + SIZE_REAL16 - 1) | 0x8000;
            grid[queue[0] as usize] |= 2;
            grid[(queue[1] & 0x7fff) as usize] |= 3;
            let mut read = 0;
            let mut write = 2;
            let mut prev_depth = 0;
            let mut depth = 0u16;
            while read < write {
                let item = *queue.get_unchecked(read);
                read += 1;
                if item & 0x8000 != prev_depth {
                    depth += 1;
                    prev_depth = item & 0x8000;
                }
                let pos = item & 0x7fff;
                if pos >= SIZE_LOGICAL16 {
                    let nxt = grid.get_unchecked_mut((pos - SIZE_LOGICAL16) as usize);
                    if *nxt & 4 == 0 {
                        if *nxt & 2 != 0 {
                            if *nxt & 1 != depth as u8 & 1 { return depth + 1; }
                        } else {
                            *nxt |= 2 | (depth as u8 & 1);
                            *queue.get_unchecked_mut(write) = (pos - SIZE_LOGICAL16) | prev_depth;
                            write += 1;
                        }
                    }
                }
                if pos < SIZE_LOGICAL16 * (SIZE_REAL16 - 1) {
                    let nxt = grid.get_unchecked_mut((pos + SIZE_LOGICAL16) as usize);
                    if *nxt & 4 == 0 {
                        if *nxt & 2 != 0 {
                            if *nxt & 1 != depth as u8 & 1 { return depth + 1; }
                        } else {
                            *nxt |= 2 | (depth as u8 & 1);
                            *queue.get_unchecked_mut(write) = (pos + SIZE_LOGICAL16) | prev_depth;
                            write += 1;
                        }
                    }
                }
                if pos & 0x7f > 0 {
                    let nxt = grid.get_unchecked_mut((pos - 1) as usize);
                    if *nxt & 4 == 0 {
                        if *nxt & 2 != 0 {
                            if *nxt & 1 != depth as u8 & 1 { return depth + 1; }
                        } else {
                            *nxt |= 2 | (depth as u8 & 1);
                            *queue.get_unchecked_mut(write) = (pos - 1) | prev_depth;
                            write += 1;
                        }
                    }
                }
                if pos & 0x7f < SIZE_REAL16 - 1 {
                    let nxt = grid.get_unchecked_mut((pos + 1) as usize);
                    if *nxt & 4 == 0 {
                        if *nxt & 2 != 0 {
                            if *nxt & 1 != depth as u8 & 1 { return depth + 1; }
                        } else {
                            *nxt |= 2 | (depth as u8 & 1);
                            *queue.get_unchecked_mut(write) = (pos + 1) | prev_depth;
                            write += 1;
                        }
                    }
                }
            }
        }
        return 0;
    }
}

unsafe fn find(parent: &mut [u16; SIZE_REAL * SIZE_REAL], node: usize) -> usize {
    let mut stack = [0u16; 64];
    let mut curr = node as u16;
    let mut ptr = 0;
    loop {
        stack[ptr] = curr;
        ptr += 1;
        let nxt = *parent.get_unchecked(curr as usize);
        if nxt == curr || nxt == 0 {
            break;
        }
        curr = nxt;
    }
    let first = curr;
    for p in 0..ptr {
        *parent.get_unchecked_mut(stack[p] as usize) = first as u16;
    }
    return first as usize;
}

unsafe fn merge(mut parent: &mut [u16; SIZE_REAL * SIZE_REAL], rank: &mut [u8; SIZE_REAL * SIZE_REAL],
    u: usize, v: usize) -> bool {
    let au = find(&mut parent, u);
    let av = find(&mut parent, v);
    if au == av { return false; }
    let ptr = rank.as_mut_ptr();
    let ru = ptr.wrapping_add(au);
    let rv = ptr.wrapping_add(av);
    *ru |= *rv & 0b11;
    *rv |= *ru & 0b11;
    if *rv & 0b11 == 3 { return true; }
    if *ru > *rv {
        parent[av] = au as u16;
    } else if *ru < *rv {
        parent[au] = av as u16;
    } else {
        parent[av] = au as u16;
        *ru += 4;
    }
    return false;
}

pub fn part2(s: &str) -> &str {
    unsafe {
        let b = s.as_bytes();
        let mut grid = [0u16; SIZE_REAL * SIZE_REAL];
        let mut rank = [0u8; SIZE_REAL * SIZE_REAL];
        let mut sep = memchr::memchr2_iter(b'\n', b',', b);
        {
            let mut last_pos = -1;
            loop {
                let pos1 = sep.next().unwrap_unchecked() as i32;
                let first_idx = pos1 - last_pos - 2;
                let pos2 = sep.next().unwrap_unchecked() as i32;
                let second_idx = pos2 - pos1 - 2;

                let mut line = u8x8::load_select_unchecked(&b[(last_pos+1) as usize..pos2 as usize],
                    mask8x8::from_bitmask((1 << (pos2 - last_pos - 1)) - 1), u8x8::splat(b'0'));
                line -= u8x8::splat(b'0');
                let mut s = u16x4::from_be_bytes(
                    line.swizzle_dyn(u8x8::from_array(
                        SHUF[first_idx as usize][second_idx as usize]))
                );
                let x = s[0] + s[1] * 10;
                s *= u16x4::from_array([1, 10, SIZE_REAL16, SIZE_REAL16 * 10]);
                let loc = s.reduce_sum();
                let edges =
                    (((loc >= SIZE_REAL16 * (SIZE_REAL16 - 1)) as u8) << 3) |
                    (((loc < SIZE_REAL16) as u8) << 2) |
                    (((x < 1) as u8) << 1) |
                    ((x >= SIZE_REAL16 - 1) as u8);
                rank[loc as usize] |= (edges | (edges >> 2)) & 0b11;
                grid[loc as usize] = loc;
                if edges & 0b1000 == 0 && grid[(loc + SIZE_REAL16) as usize] != 0 {
                    if merge(&mut grid, &mut rank, (loc + SIZE_REAL16) as usize, loc as usize) {
                        break;
                    }
                }
                if edges & 0b0100 == 0 && grid[(loc - SIZE_REAL16) as usize] != 0 {
                    if merge(&mut grid, &mut rank, (loc - SIZE_REAL16) as usize, loc as usize) {
                        break;
                    }
                }
                if edges & 0b0010 == 0 && grid[loc as usize - 1] != 0 {
                    if merge(&mut grid, &mut rank, loc as usize - 1, loc as usize) {
                        break;
                    }            
                }
                if edges & 0b0001 == 0 && grid[loc as usize + 1] != 0 {
                    if merge(&mut grid, &mut rank, loc as usize + 1, loc as usize) {
                        break;
                    }
                }
                if edges & 0b1010 == 0 && grid[(loc - 1 + SIZE_REAL16) as usize] != 0 {
                    if merge(&mut grid, &mut rank, (loc - 1 + SIZE_REAL16) as usize, loc as usize) {
                        break;
                    }
                }
                if edges & 0b1001 == 0 && grid[(loc + 1 + SIZE_REAL16) as usize] != 0 {
                    if merge(&mut grid, &mut rank, (loc + 1 + SIZE_REAL16) as usize, loc as usize) {
                        break;
                    }
                }
                if edges & 0b0110 == 0 && grid[(loc - 1 - SIZE_REAL16) as usize] != 0 {
                    if merge(&mut grid, &mut rank, (loc - 1 - SIZE_REAL16) as usize, loc as usize) {
                        break;
                    }
                }
                if edges & 0b0101 == 0 && grid[(loc + 1 - SIZE_REAL16) as usize] != 0 {
                    if merge(&mut grid, &mut rank, (loc + 1 - SIZE_REAL16) as usize, loc as usize) {
                        break;
                    }
                }
                last_pos = pos2;
            }
            last_pos += 1;
            return &s[
                last_pos as usize..
                last_pos as usize +
                memchr::memchr(b'\n', &b[last_pos as usize..]).unwrap_unchecked()
            ];
        }
    }
}