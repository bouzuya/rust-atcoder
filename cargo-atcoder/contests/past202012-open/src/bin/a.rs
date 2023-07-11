use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
    };
    let mut max_count_o = 0;
    let mut max_count_x = 0;
    let mut count_o = 0;
    let mut count_x = 0;
    for &c in s.iter() {
        match c {
            'o' => {
                count_x = 0;
                count_o += 1;
                max_count_o = max(max_count_o, count_o);
            }
            'x' => {
                count_o = 0;
                count_x += 1;
                max_count_x = max(max_count_x, count_x);
            }
            _ => unreachable!("unknown"),
        }
    }
    let ans = if max_count_o >= 3 {
        "o"
    } else if max_count_x >= 3 {
        "x"
    } else {
        "draw"
    };
    println!("{}", ans);
}
