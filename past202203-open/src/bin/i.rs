use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [(i64, i64); n],
        mut t: [(i64, i64); n],
    };

    s.sort_by_key(|&(x, y)| (x, y));
    t.sort_by_key(|&(x, y)| (x, y));
    if s == t {
        println!("Yes");
        return;
    }

    s.sort_by_key(|&(x, y)| (y, x));
    t.sort_by_key(|&(x, y)| (y, x));
    if s == t {
        println!("Yes");
        return;
    }

    s.sort_by_key(|&(x, y)| (x, y));
    t.sort_by_key(|&(x, y)| (x, Reverse(y)));
    let mut y = None;
    let mut ok = true;
    for ((s_x, s_y), (t_x, t_y)) in s.iter().copied().zip(t.iter().copied()) {
        if s_x != t_x {
            ok = false;
            break;
        }
        match y {
            Some(y) => {
                if y != s_y + t_y {
                    ok = false;
                    break;
                }
            }
            None => {
                y = Some(s_y + t_y);
            }
        }
    }
    if ok {
        println!("Yes");
        return;
    }

    s.sort_by_key(|&(x, y)| (y, x));
    t.sort_by_key(|&(x, y)| (y, Reverse(x)));
    let mut x = None;
    let mut ok = true;
    for ((s_x, s_y), (t_x, t_y)) in s.iter().copied().zip(t.iter().copied()) {
        if s_y != t_y {
            ok = false;
            break;
        }
        match x {
            Some(x) => {
                if x != s_x + t_x {
                    ok = false;
                    break;
                }
            }
            None => {
                x = Some(s_x + t_x);
            }
        }
    }
    if ok {
        println!("Yes");
        return;
    }

    println!("No");
}
