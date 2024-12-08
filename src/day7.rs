unsafe fn permute1(num: u64, nums: &Vec<u32>, idx: usize) -> bool {
    let next = nums[idx] as u64;
    let div = num / next;
    let rem = num % next; // please optimize this please

    return
        idx == 0 && num == next ||
        idx > 0 && (
            (rem == 0 && permute1(div, &nums, idx - 1)) ||
            (num >= next && permute1(num - next, &nums, idx - 1))
        );
}

pub fn part1(s: &str) -> u64 {
    unsafe {       
        let mut nums = vec![0u32];
        return s.split('\n').fold(0u64, |ans, line|{
            if line.is_empty() { return ans }
            let mut res = 0;
            nums.clear();
            line.split(' ').for_each(|num| {
                if num.ends_with(':') {
                    //res = atoi_simd::parse_any::<u64>()
                    res = num.replacen(':', "", 1).parse::<u64>().unwrap_unchecked();
                } else {
                    nums.push(num.parse::<u32>().unwrap_unchecked());
                }
            });
            return ans + if permute1(res, &nums, nums.len() - 1) { res } else { 0 };
        });
    }
}

unsafe fn permute2(num: u64, nums: &Vec<u32>, idx: usize) -> bool {
    let next = nums[idx] as u64;
    if idx == 0 { return num == next }

    let div = num / next;
    let rem = num % next;
    if (rem == 0 && permute2(div, &nums, idx - 1)) ||
           (num >= next && permute2(num - next, &nums, idx - 1)) { return true; }
    let log = if next >= u64::pow(10, 6) {
        if next >= u64::pow(10, 10) {
            if next >= u64::pow(10, 12) {
                if next >= u64::pow(10, 13) {
                    if next >= u64::pow(10, 14) { u64::pow(10, 15) }
                    else { u64::pow(10, 14) }
                } else { u64::pow(10, 13) }
            } else {
                if next >= u64::pow(10, 11) { u64::pow(10, 12) }
                else { u64::pow(10, 11) }
            }
        } else {
            if next >= u64::pow(10, 8) {
                if next >= u64::pow(10, 9) { u64::pow(10, 10) }
                else { u64::pow(10, 9) }
            } else {
                if next >= u64::pow(10, 7) { u64::pow(10, 8) }
                else { u64::pow(10, 7) }
            }
        }
    } else {
        if next >= u64::pow(10, 4) {
            if next >= u64::pow(10, 5) { u64::pow(10, 6) }
            else { u64::pow(10, 5) }
        } else {
            if next >= u64::pow(10, 2) {
                if next >= u64::pow(10, 3) { u64::pow(10, 4) } else { u64::pow(10, 3) }
            } else {
                if next >= u64::pow(10, 1) { u64::pow(10, 2) }
                else { u64::pow(10, 1) }
            }
        }
    };
    let div_ = num / log;
    let rem_ = num % log;
    return rem_ == next && permute2(div_, &nums, idx - 1);
}

pub fn part2(s: &str) -> u64 {
    unsafe {       
        let mut nums = vec![0u32];
        return s.split('\n').fold(0u64, |ans, line|{
            if line.is_empty() { return ans }
            let mut res = 0;
            nums.clear();
            line.split(' ').for_each(|num| {
                if num.ends_with(':') {
                    res = num.replacen(':', "", 1).parse::<u64>().unwrap_unchecked();
                } else {
                    nums.push(num.parse::<u32>().unwrap_unchecked());
                }
            });
            return ans + if permute2(res, &nums, nums.len() - 1) { res } else { 0 };
        });
    }
}
