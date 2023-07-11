use proconio::input;
use proconio::marker::Chars;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let inf = 100 * 100;
    let mut d = vec![vec![inf; w]; h];
    let mut q = std::collections::VecDeque::new();
    d[0][0] = if s[0][0] == '#' { 1 } else { 0 };
    q.push_back((0, 0));
    while let Some((y, x)) = q.pop_front() {
        if x + 1 < w {
            if chmin!(
                d[y][x + 1],
                d[y][x] + if s[y][x] == s[y][x + 1] { 0 } else { 1 }
            ) {
                q.push_back((y, x + 1));
            }
        }
        if y + 1 < h {
            if chmin!(
                d[y + 1][x],
                d[y][x] + if s[y][x] == s[y + 1][x] { 0 } else { 1 }
            ) {
                q.push_back((y + 1, x));
            }
        }
    }
    let ans = (d[h - 1][w - 1] + if s[h - 1][w - 1] == '#' { 1 } else { 0 }) / 2;
    println!("{}", ans);
}
