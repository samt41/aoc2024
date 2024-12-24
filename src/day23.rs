use std::intrinsics::unaligned_volatile_load;
use std::str;

pub fn part1(s: &str) -> u16 {
    unsafe {
        let mut vis = [0u32; 6656];
        let b = s.as_bytes();
        let N = b.len() / 6;
        let mut ptr = s.as_ptr();
        for _ in 0..N {
            if *ptr == b't' {
                let word = unaligned_volatile_load::<u16>(ptr.wrapping_add(3) as *const u16) - 0x6161;
                vis[word as usize] |= 1 << (*(ptr.wrapping_add(1)) - b'a');
            } else
            if *(ptr.wrapping_add(3)) == b't' {
                let word = unaligned_volatile_load::<u16>(ptr as *const u16) - 0x6161;
                vis[word as usize] |= 1 << (*(ptr.wrapping_add(4)) - b'a');
            }
            ptr = ptr.add(6);
        }
        ptr = s.as_ptr();
        let mut ans = 0u16;
        for _ in 0..N {
            let word1 = unaligned_volatile_load::<u16>(ptr as *const u16) - 0x6161;
            let word2 = unaligned_volatile_load::<u16>(ptr.wrapping_add(3) as *const u16) - 0x6161;
            let shared = vis[word1 as usize] & vis[word2 as usize];
            ans += shared.count_ones() as u16;
            ptr = ptr.add(6);
        }
        return ans;
    }
}

unsafe fn bron_kerbosch(r: *mut u64, p: *mut u64, x: *mut u64, adj: *mut u64, curr: *mut u64, best: &mut u32) {
    let ap;
    {
        let mut _ap = 0;
        for i in 0..12 { _ap += (*p.wrapping_add(i)).count_ones(); }
        ap = _ap;
    }
    let ax;
    {
        let mut _ax = 0;
        for i in 0..12 { _ax += (*p.wrapping_add(i)).count_ones(); }
        ax = _ax;
    }
    if ap + ax == 0 {
        // clique
        let mut ar = 0;
        for i in 0..12 { ar += (*r.wrapping_add(i)).count_ones(); }
        if ar > *best {
            std::ptr::copy_nonoverlapping(r, curr, 12);
            *best = ar;
        }
        return;
    }
    let mut pivot = 0;
    for i in 0..12 {
        let curr = *p.wrapping_add(i) | *x.wrapping_add(i);
        if curr != 0 {
            pivot += curr.trailing_zeros();
            break;
        }
        pivot += 64;
    }
    let adjpiv = adj.wrapping_add(12 * pivot as usize);
    for i in 0..12 {
        let mut non_neighbours = *p.wrapping_add(i) & !*adjpiv.wrapping_add(i);
        while non_neighbours != 0 {
            let item = 1u64 << non_neighbours.trailing_zeros();
            let in_item = *r.wrapping_add(i) & item;
            *r.wrapping_add(i) |= item;

            let p_next = [0u64; 12].as_mut_ptr();
            let x_next = [0u64; 12].as_mut_ptr();
            let adj2 = adj.wrapping_add(12 * (i * 64 + non_neighbours.trailing_zeros() as usize));
            for j in 0..12 {
                *p_next.wrapping_add(j) = *p.wrapping_add(j) & *adj2.wrapping_add(j);
                *x_next.wrapping_add(j) = *x.wrapping_add(j) & *adj2.wrapping_add(j);
            }
            bron_kerbosch(r, p_next, x_next, adj, curr, best);

            *p.wrapping_add(i) ^= item;
            *x.wrapping_add(i) |= item;
            *r.wrapping_add(i) ^= !in_item & item;
            non_neighbours &= non_neighbours - 1;
        }
    }
}

pub fn part2(s: &str) -> String {
    unsafe {
        let mut adjacency = [0u64; 26 * 26 * 12];
        let b = s.as_bytes();
        let N = b.len() / 6;
        let mut ptr = s.as_ptr();
        let p = [0u64; 12].as_mut_ptr();
        let adj = adjacency.as_mut_ptr();
        for _ in 0..N {
            let mut word1 = unaligned_volatile_load::<u16>(ptr as *const u16) - 0x6161;
            let mut word2 = unaligned_volatile_load::<u16>(ptr.wrapping_add(3) as *const u16) - 0x6161;
            word1 = ((word1 & 0xff) * 26) + ((word1 & 0xff00) >> 8);
            word2 = ((word2 & 0xff) * 26) + ((word2 & 0xff00) >> 8);
            *adj.wrapping_add((word1 * 12 + (word2 >> 6)) as usize) |= 1 << (word2 & 63);
            *adj.wrapping_add((word2 * 12 + (word1 >> 6)) as usize) |= 1 << (word1 & 63);
            *p.wrapping_add((word1 >> 6) as usize) |= 1 << (word1 & 63);
            *p.wrapping_add((word2 >> 6) as usize) |= 1 << (word2 & 63);
            ptr = ptr.add(6);
        }

        let curr = [0u64; 12].as_mut_ptr();
        let mut best = 0;
        let r = [0u64; 12].as_mut_ptr();
        let x = [0u64; 12].as_mut_ptr();
        bron_kerbosch(r, p, x, adj, curr, &mut best);
        let mut ans = vec![0u8; 38];
        let mut idx = 0;
        let mut output_ptr = 0;
        for i in 0..12 {
            let mut val = *curr.wrapping_add(i);
            while val != 0 {
                let it = idx + val.trailing_zeros();
                ans[output_ptr] = (it / 26) as u8 + b'a';
                ans[output_ptr + 1] = (it % 26) as u8 + b'a';
                if output_ptr + 2 < 38 {
                    ans[output_ptr + 2] = b',';
                }
                output_ptr += 3;
                val &= val - 1;
            }
            idx += 64;
        }
        return String::from_utf8_unchecked(ans);
    }
}