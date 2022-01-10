use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        t: Chars,
    };
    let mut ans = 0;
    let mut r = 0;
    for l in 0..n {
        if s[l] == t[l] {
            continue;
        }

        match s[r.max(l + 1)..].iter().position(|&c| c == '1') {
            Some(found) => {
                r = found + r.max(l + 1);
            }
            None => {
                println!("-1");
                return;
            }
        }

        if r != l + 1 {
            s.swap(l + 1, r);
            ans += r - (l + 1);
        }

        {
            let (a, b) = match t[l] {
                '0' => ('0', '0'),
                '1' => ('1', '0'),
                _ => unreachable!(),
            };
            s[l] = a;
            s[l + 1] = b;
            ans += 1;
        }
    }
    println!("{}", ans);
}
