use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let n = s.len();
    let mut d = vec![0; (n + 1) * 2 + 100];
    let mut c = vec![0; (n + 1) * 2 + 100];
    c[0] = 5;
    let mut ct = 0;
    let mut combo = 0_usize;
    for i in 0..2 * n {
        if i > 0 {
            c[i] += c[i - 1];
        }
        let s_i = if i % 2 == 0 { s[i / 2] } else { '-' };
        if d[i] > 0 {
            combo += 1;
        }
        if ct > 0 {
            ct -= 1;
            continue;
        }
        match s_i {
            '-' => {}
            'N' => {
                if c[i] >= 1 {
                    c[i + 1] -= 1;
                    c[i + 1 + 2 + 10] += 1;
                    d[i + 1 + 2] += 10 * (10 + combo / 10);
                }
            }
            'C' => {
                if c[i] >= 3 {
                    ct = 5;
                    c[i + 1] -= 3;
                    c[i + 5 + 2 + 10] += 3;
                    d[i + 5 + 2] += 50 * (10 + combo / 10);
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = d.into_iter().sum::<usize>() / 10;
    println!("{}", ans);
}
