use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut stack = vec![];
    let mut count = vec![0_i64; n + 1];
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            '(' => {
                stack.push(i);
            }
            ')' => {
                if let Some(j) = stack.pop() {
                    count[j] += 1;
                    count[i + 1] -= 1;
                }
            }
            _ => {}
        }
    }

    for i in 0..n {
        count[i + 1] += count[i];
    }
    let mut ans = String::new();
    for (i, s_i) in s.iter().copied().enumerate() {
        if count[i] == 0 {
            ans.push(s_i);
        }
    }
    println!("{}", ans);
}
