use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    };

    let mut count = 0_usize;
    for i in x..h {
        if s[i][y] == '#' {
            break;
        }
        count += 1;
    }
    for i in (0..x).rev() {
        if s[i][y] == '#' {
            break;
        }
        count += 1;
    }
    for j in y..w {
        if s[x][j] == '#' {
            break;
        }
        count += 1;
    }
    for j in (0..y).rev() {
        if s[x][j] == '#' {
            break;
        }
        count += 1;
    }

    let ans = count - 1;
    println!("{}", ans);
}
