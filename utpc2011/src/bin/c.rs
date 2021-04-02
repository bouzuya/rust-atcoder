use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn f(l: &Vec<char>, r: &Vec<char>, i_l: usize, i_r: usize, len: usize) -> usize {
    if i_l == l.len() || i_r == r.len() {
        return len;
    }
    if l[i_l] == 'i' || l[i_l] == 'w' {
        return f(l, r, i_l + 1, i_r, len);
    }
    let c = match l[i_l] {
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        '[' => ']',
        ']' => '[',
        _ => unreachable!("invalid char"),
    };
    let mut found = None;
    for j in i_r..r.len() {
        if r[j] == c {
            found = Some(j);
            break;
        }
    }
    max(
        match found {
            Some(j) => f(l, r, i_l + 1, j + 1, len + 1),
            None => len,
        },
        f(l, r, i_l + 1, i_r, len),
    )
}

fn main() {
    input! {
        s: Chars,
    };
    let mut max_len = 0;
    let n = s.len();
    for i in 0..n {
        if s[i] != 'w' {
            continue;
        }

        let mut left_i = None;
        for j in (0..i).rev() {
            if s[j] == 'i' {
                left_i = Some(j);
                break;
            }
        }
        if left_i.is_none() {
            continue;
        }
        let mut right_i = None;
        for j in i + 1..n {
            if s[j] == 'i' {
                right_i = Some(j);
                break;
            }
        }
        if right_i.is_none() {
            continue;
        }
        let l = s[0..left_i.unwrap()].iter().cloned().collect::<Vec<char>>();
        let r = s[right_i.unwrap() + 1..n]
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<char>>();
        let len = 3 + f(&l, &r, 0, 0, 0) * 2;
        max_len = max(max_len, len);
    }
    let ans = max_len;
    println!("{}", ans);
}
