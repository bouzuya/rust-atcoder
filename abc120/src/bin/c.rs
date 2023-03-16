use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut count = 0_usize;
    let mut stack = vec![];
    for s_i in s {
        if stack.is_empty() {
            stack.push(s_i);
        } else {
            let last = stack.pop().unwrap();
            if last != s_i {
                count += 1;
            } else {
                stack.push(last);
                stack.push(s_i);
            }
        }
    }
    let ans = count * 2;
    println!("{}", ans);
}
