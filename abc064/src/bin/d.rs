use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    };

    let mut stack = vec![];
    let mut t = vec![];
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            '(' => {
                stack.push(i);
            }
            ')' => {
                stack.pop();
            }
            _ => unreachable!(),
        }
        t.push(s_i);
    }
    while let Some(_) = stack.pop() {
        t.push(')');
    }
    t.reverse();
    s = t
        .into_iter()
        .map(|t_i| match t_i {
            '(' => ')',
            ')' => '(',
            _ => unreachable!(),
        })
        .collect::<Vec<char>>();
    let mut t = vec![];
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            '(' => {
                stack.push(i);
            }
            ')' => {
                stack.pop();
            }
            _ => unreachable!(),
        }
        t.push(s_i);
    }
    while let Some(_) = stack.pop() {
        t.push(')');
    }
    t.reverse();
    let ans = t
        .into_iter()
        .map(|t_i| match t_i {
            '(' => ')',
            ')' => '(',
            _ => unreachable!(),
        })
        .collect::<String>();
    println!("{}", ans);
}
