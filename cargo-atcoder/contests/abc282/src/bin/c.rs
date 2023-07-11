use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut ans = vec![];
    let mut stack = vec![];
    for s_i in s {
        if s_i == '"' {
            if stack.is_empty() {
                stack.push('"');
            } else {
                stack.pop();
            }
            ans.push('"');
        } else {
            if s_i == ',' {
                if stack.is_empty() {
                    ans.push('.');
                } else {
                    ans.push(',');
                }
            } else {
                ans.push(s_i)
            }
        }
    }
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
