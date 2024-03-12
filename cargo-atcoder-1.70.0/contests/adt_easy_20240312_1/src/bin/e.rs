use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        m: usize,
        s: Chars,
    };
    let mut max = 0_usize;
    let mut count1 = 0;
    let mut count2 = 0;
    for c in s.iter().copied().rev() {
        match c {
            '0' => {
                count1 = 0;
                count2 = 0;
            }
            '1' => {
                if count1 < m {
                    count1 += 1;
                } else {
                    count2 += 1;
                    max = max.max(count2);
                }
            }
            '2' => {
                count2 += 1;
                max = max.max(count2);
            }
            _ => unreachable!(),
        }
    }
    let ans = max;
    println!("{}", ans);
}
