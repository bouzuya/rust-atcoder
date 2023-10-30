use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut stack = vec![];
    for s_i in s {
        match s_i {
            '(' => stack.push(s_i),
            ')' => {
                if stack.pop().is_none() {
                    println!("No");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = stack.is_empty();
    println!("{}", if ans { "Yes" } else { "No" });
}
