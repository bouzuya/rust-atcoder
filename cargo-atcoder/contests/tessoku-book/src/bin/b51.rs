use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = vec![];
    let mut stack = vec![];
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            '(' => stack.push(i),
            ')' => {
                ans.push((stack.pop().unwrap(), i));
            }
            _ => unreachable!(),
        }
    }

    for (o, c) in ans {
        println!("{} {}", o + 1, c + 1);
    }
}
