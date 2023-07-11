use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            _n: usize,
            k: usize,
            s: Chars,
        }

        let count1 = s.iter().copied().filter(|s_i| s_i == &'1').count();
        let mut count = 0_usize;
        let mut c0 = 0_usize;
        let mut c1 = 0_usize;
        for s_i in s.iter().copied().take(k) {
            match s_i {
                '0' => c0 += 1,
                '1' => c1 += 1,
                '?' => {}
                _ => unreachable!(),
            }
        }
        count += if c0 == 0 && c1 == count1 { 1 } else { 0 };
        for (i, s_i) in s.iter().copied().enumerate().skip(k) {
            match s_i {
                '0' => c0 += 1,
                '1' => c1 += 1,
                '?' => {}
                _ => unreachable!(),
            }
            match s[i - k] {
                '0' => c0 -= 1,
                '1' => c1 -= 1,
                '?' => {}
                _ => unreachable!(),
            }
            count += if c0 == 0 && c1 == count1 { 1 } else { 0 };
        }
        let ans = count == 1;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
