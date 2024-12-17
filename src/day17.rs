use std::io::Write;
use std::str;

pub fn part1(s: &str) -> String {
    unsafe {
        let b = s.as_bytes();
        let bl = b.len();
        let mut output = vec![0u8; 0];
        let mut reg = [0, 1, 2, 3, 0u32, 0u32, 0u32];
        let base_reg = reg.as_ptr();
        let base_reg_mut = reg.as_mut_ptr();
        let (regA, regB, regC) = 
            (base_reg_mut.wrapping_add(4), base_reg_mut.wrapping_add(5), base_reg_mut.wrapping_add(6));
        let mut ptr: usize;
        let mut itoabuf = itoa::Buffer::new();
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
            let ins = b[ptrp] - b'0';
            let item = b[ptrp + 2] - b'0';
            let combo_lit = if (1 << ins) & (0b11100101) != 0 { *base_reg.wrapping_add(item as usize) } else { item as u32 };
            ptrp += 4;
            match ins {
                0 => *regA >>= combo_lit,
                1 => *regB ^= combo_lit,
                2 => *regB = combo_lit & 0b111,
                3 => if *regA != 0 { ptrp = ptr + (combo_lit << 2) as usize },
                4 => *regB ^= *regC,
                5 => {
                    output.write_all(itoabuf.format(combo_lit & 0b111).as_bytes()).unwrap_unchecked();
                    output.push(b',');
                },
                v => *base_reg_mut.wrapping_add(v as usize - 1) = *regA >> combo_lit,
            }
        }
        {
            output.pop();
        }
        return String::from_utf8(output).unwrap_unchecked();
    }
}

pub fn part2(_s: &str) -> u32 {
    0
}