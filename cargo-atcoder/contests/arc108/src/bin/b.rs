use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut stack = vec![];
    for c in s {
        if c == 'x' && stack.len() >= 2 {
            let c1 = stack.pop().unwrap();
            if c1 == 'o' {
                let c2 = stack.pop().unwrap();
                if c2 != 'f' {
                    stack.push(c2);
                    stack.push(c1);
                    stack.push(c);
                }
            } else {
                stack.push(c1);
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    let ans = stack.len();
    println!("{}", ans);
}
