use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q],
    };
    let mut rev = false;
    for (t_i, a_i, b_i) in tab {
        match t_i {
            1 => {
                let a_i = a_i - 1;
                let b_i = b_i - 1;
                let (l, r) = if rev {
                    if a_i < n && b_i < n {
                        (n + a_i, n + b_i)
                    } else if a_i < n && n <= b_i {
                        (n + a_i, b_i - n)
                    } else if n <= a_i && n <= b_i {
                        (a_i - n, b_i - n)
                    } else {
                        unreachable!()
                    }
                } else {
                    (a_i, b_i)
                };
                let t = s[l];
                s[l] = s[r];
                s[r] = t;
            }
            2 => {
                rev = !rev;
            }
            _ => unreachable!(),
        }
    }
    let ans = if rev {
        format!(
            "{}{}",
            s.iter().skip(n).collect::<String>(),
            s.iter().take(n).collect::<String>(),
        )
    } else {
        s.iter().collect::<String>()
    };
    println!("{}", ans);
}
