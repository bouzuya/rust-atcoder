use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q],
    };

    let mut at = vec![0; n];
    let mut ts = vec![];
    for (i, (t, x, c)) in txc.iter().copied().enumerate() {
        let i = i + 1;
        match t {
            1 => {
                let x = x - 1;
                s[x] = c;
                at[x] = i;
            }
            2 => ts.push((i, false)),
            3 => ts.push((i, true)),
            _ => unreachable!(),
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        match ts.last() {
            None => ans.push(s[i]),
            Some(&(a, b)) => {
                if at[i] < a {
                    ans.push(if b {
                        s[i].to_uppercase().to_string().chars().next().unwrap()
                    } else {
                        s[i].to_lowercase().to_string().chars().next().unwrap()
                    });
                } else {
                    ans.push(s[i]);
                }
            }
        }
    }

    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
