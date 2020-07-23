use proconio::input;
use proconio::marker::Chars;

fn length(c: &[char], l: &[char], r: &[char]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < c.len() {
        if i + 1 >= c.len() {
            i += 1;
            len += 1;
        } else if c[i] == l[0] && c[i + 1] == l[1] {
            i += 2;
            len += 1;
        } else if c[i] == r[0] && c[i + 1] == r[1] {
            i += 2;
            len += 1;
        } else {
            i += 1;
            len += 1;
        }
    }
    len
}

fn main() {
    input! {
        _: usize,
        c: Chars,
    };
    let buttons = ['A', 'B', 'X', 'Y'];
    let mut ans = c.len();
    for &l_b_1 in buttons.iter() {
        for &l_b_2 in buttons.iter() {
            let l = [l_b_1, l_b_2];
            for &r_b_1 in buttons.iter() {
                for &r_b_2 in buttons.iter() {
                    let r = [r_b_1, r_b_2];
                    ans = std::cmp::min(ans, length(&c, &l, &r));
                }
            }
        }
    }
    println!("{}", ans);
}
