use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut in_pipe = false;
    let mut ans = vec![];
    for c in s {
        if !in_pipe {
            if c == '|' {
                in_pipe = true;
            } else {
                ans.push(c);
            }
        } else {
            if c == '|' {
                in_pipe = false;
            } else {
                // do nothing
            }
        }
    }
    let ans = ans.into_iter().collect::<String>();
    println!("{}", ans);
}
