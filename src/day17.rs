pub fn part1(s: &str) -> String {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut output = vec![0u16; 0];
        let mut reg = [0, 1, 2, 3, 0u32, 0u32, 0u32];
        let base_reg = reg.as_ptr();
        let base_reg_mut = reg.as_mut_ptr();
        let (regA, regB, regC) = 
            (base_reg_mut.wrapping_add(4), base_reg_mut.wrapping_add(5), base_reg_mut.wrapping_add(6));
        let mut ptr: usize;
        {
            let mut x;
            ptr = memchr::memchr(b':', b).unwrap_unchecked() + 2;
            (*regA, x) = atoi_simd::parse_any_pos::<u32>(&b[ptr..]).unwrap_unchecked();
            ptr += x + 1;
            ptr += memchr::memchr(b':', &b[ptr..]).unwrap_unchecked() + 2;
            (*regB, x) = atoi_simd::parse_any_pos::<u32>(&b[ptr..]).unwrap_unchecked();
            ptr += x + 1;
            ptr += memchr::memchr(b':', &b[ptr..]).unwrap_unchecked() + 2;
            (*regC, x) = atoi_simd::parse_any_pos::<u32>(&b[ptr..]).unwrap_unchecked();
            ptr += x + 2;
            ptr += memchr::memchr(b':', &b[ptr..]).unwrap_unchecked() + 2;
        }
        let mut ptrp = ptr;
        while ptrp < bl {
            let ins = b.get_unchecked(ptrp) - b'0';
            let item = b.get_unchecked(ptrp + 2) - b'0';
            let combo = *base_reg.wrapping_add(item as usize);
            ptrp += 4;
            match ins {
                0 => *regA >>= combo,
                1 => *regB ^= item as u32,
                2 => *regB = combo & 0b111,
                3 => if *regA != 0 { ptrp = ptr + (item << 2) as usize },
                4 => *regB ^= *regC,
                5 => output.push(((combo as u16 & 0b111) + b'0' as u16) | (b',' as u16) << 8),
                6 => *regB = *regA >> combo,
                7 => *regC = *regA >> combo,
                _ => break,
            }
        }
        {
            let l = output.len();
            let cap = output.capacity();
            let pt = output.as_mut_ptr();
            let tempv = Vec::from_raw_parts(pt as *mut u8, (l << 1) - 1, cap);
            std::mem::forget(output);
            return String::from_utf8_unchecked(tempv);
        }
    }
}

pub fn part2(_s: &str) -> u32 {
    0
}