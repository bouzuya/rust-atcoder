use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        y: Usize1,
        x: Usize1,
        s: [Chars; h],
    };
    let mut count = 1;
    for i in (0..y).rev() {
        if s[i][x] == '.' {
            count += 1;
        } else {
            break;
        }
    }
    for i in y + 1..h {
        if s[i][x] == '.' {
            count += 1;
        } else {
            break;
        }
    }
    for j in (0..x).rev() {
        if s[y][j] == '.' {
            count += 1;
        } else {
            break;
        }
    }
    for j in x + 1..w {
        if s[y][j] == '.' {
            count += 1;
        } else {
            break;
        }
    }
    let ans = count;
    println!("{}", ans);
}
