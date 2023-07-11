use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let d = n + 1;
    let mut t = vec![];
    for s_i in s.iter().copied() {
        match s_i {
            'o' => {
                let l = t.len();
                if let Some(&c) = t.get(l.saturating_sub(1)) {
                    if c == d {
                        t.push(1);
                    } else {
                        t[l.saturating_sub(1)] = c + 1;
                    }
                } else {
                    t.push(1);
                }
            }
            '-' => {
                t.push(d);
            }
            _ => unreachable!(),
        }
    }
    let mut max = 0_usize;
    for (i, t_i) in t.iter().copied().enumerate() {
        if t_i != d {
            continue;
        }

        max = max
            .max(if i > 0 && t[i - 1] != d { t[i - 1] } else { 0 })
            .max(if i + 1 < t.len() && t[i + 1] != d {
                t[i + 1]
            } else {
                0
            });
    }
    let ans = if max == 0 { -1 } else { max as i64 };
    println!("{}", ans);
}
