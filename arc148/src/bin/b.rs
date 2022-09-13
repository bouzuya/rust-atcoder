use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    match s.iter().position(|s_i| s_i == &'p') {
        None => {
            println!("{}", s.into_iter().collect::<String>());
        }
        Some(l) => {
            let f = |r: usize| -> String {
                let mut t = s.clone();
                for i in l..r + 1 {
                    let j = r - (i - l);
                    if s[j] == 'd' {
                        t[i] = 'p';
                    } else {
                        t[i] = 'd';
                    }
                }
                t.into_iter().collect::<String>()
            };
            let mut ans = s.iter().collect::<String>();
            for r in l..n {
                ans = ans.min(f(r));
            }
            println!("{}", ans);
        }
    }
}
