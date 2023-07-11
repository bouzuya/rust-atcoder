// 解説 AC
// <https://learn.nakaken88.com/atcoder-regular-contest-012-c-20201226/>
use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn is_win(b: &Vec<Vec<char>>, c: char) -> bool {
    let mut res = 0;
    for i in 0..19 {
        for j in 0..19 {
            if b[i][j] != c {
                continue;
            }

            // -
            let mut r = 0;
            for x in 0..19 {
                if j + x < 19 && b[i][j + x] == c {
                    r += 1;
                } else {
                    break;
                }
            }
            res = max(res, r);
            // /
            let mut r = 0;
            for x in 0..19 {
                if i >= x && j + x < 19 && b[i - x][j + x] == c {
                    r += 1;
                } else {
                    break;
                }
            }
            res = max(res, r);
            // \
            let mut r = 0;
            for x in 0..19 {
                if i + x < 19 && j + x < 19 && b[i + x][j + x] == c {
                    r += 1;
                } else {
                    break;
                }
            }
            res = max(res, r);
            // |
            let mut r = 0;
            for x in 0..19 {
                if i + x < 19 && b[i + x][j] == c {
                    r += 1;
                } else {
                    break;
                }
            }
            res = max(res, r);
        }
    }
    res >= 5
}

fn main() {
    input! {
        mut b: [Chars; 19]
    };
    let mut c_b = 0;
    let mut c_w = 0;
    for i in 0..19 {
        for j in 0..19 {
            match b[i][j] {
                'o' => {
                    c_b += 1;
                }
                'x' => {
                    c_w += 1;
                }
                '.' => {}
                _ => unreachable!("invalid char"),
            }
        }
    }
    if c_b == 0 && c_w == 0 {
        println!("YES");
        return;
    }
    if c_b != c_w && c_b != c_w + 1 {
        println!("NO");
        return;
    }
    let w_b = is_win(&b, 'o');
    let w_w = is_win(&b, 'x');
    if w_b && w_w {
        println!("NO");
        return;
    }
    if w_b && c_b == c_w {
        println!("NO");
        return;
    }
    if w_w && c_b == c_w + 1 {
        println!("NO");
        return;
    }
    let mut ok = false;
    for i in 0..19 {
        for j in 0..19 {
            if b[i][j] == '.' {
                continue;
            }
            let t = b[i][j];
            b[i][j] = '.';
            ok |= !is_win(&b, 'o') && !is_win(&b, 'x');
            b[i][j] = t;
        }
    }
    println!("{}", if ok { "YES" } else { "NO" });
}
