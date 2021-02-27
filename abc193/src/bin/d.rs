use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    };
    let mut count_s = vec![0; 10];
    let mut count_t = vec![0; 10];
    let mut count = vec![0; 10];
    for &c in s.iter().take(4) {
        count[(c as u8 - '0' as u8) as usize] += 1;
        count_s[(c as u8 - '0' as u8) as usize] += 1;
    }
    for &c in t.iter().take(4) {
        count[(c as u8 - '0' as u8) as usize] += 1;
        count_t[(c as u8 - '0' as u8) as usize] += 1;
    }
    let mut all = 0_usize;
    let mut win = 0_usize;
    for i in 1..=9 {
        for j in 1..=9 {
            let is_ng = if i == j {
                count[i] + 2 > k
            } else {
                count[i] + 1 > k || count[j] + 1 > k
            };
            if is_ng {
                continue;
            }
            let p = if i == j {
                (k - count[i]) * (k - count[j] - 1)
            } else {
                (k - count[i]) * (k - count[j])
            };
            all += p;

            let mut sum_s = 0_usize;
            for l in 1..=9 {
                sum_s += l * 10_usize.pow(count_s[l] + if i == l { 1 } else { 0 });
            }
            let mut sum_t = 0_usize;
            for l in 1..=9 {
                sum_t += l * 10_usize.pow(count_t[l] + if j == l { 1 } else { 0 })
            }
            if sum_s > sum_t {
                win += p;
            } else {
                continue;
            }
        }
    }
    let ans = win as f64 / all as f64;
    println!("{}", ans);
}
