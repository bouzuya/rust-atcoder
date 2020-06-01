use proconio::input;
use proconio::marker::Chars;

fn dfs(
    used: &mut Vec<Vec<bool>>,
    c: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    s: (usize, usize),
    g: (usize, usize),
) -> bool {
    if s == g {
        return true;
    }
    let (x, y) = s;
    if used[y][x] {
        return false;
    }
    used[y][x] = true;
    let d = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for &(d_x, d_y) in d.iter() {
        let x_n = x as i64 + d_x;
        let y_n = y as i64 + d_y;
        if (0..h as i64).contains(&y_n) && (0..w as i64).contains(&x_n) {
            let (x_n, y_n) = (x_n as usize, y_n as usize);
            if dfs(used, c, h, w, (x_n, y_n), g) {
                return true;
            }
        }
    }
    false
}
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut used = vec![vec![false; w]; h];
    let mut s = (0, 0);
    let mut g = (0, 0);
    for y in 0..h {
        for x in 0..w {
            match c[y][x] {
                's' => s = (x, y),
                'g' => g = (x, y),
                '.' => {}
                '#' => used[y][x] = true,
                _ => unreachable!(),
            }
        }
    }
    let ans = dfs(&mut used, &c, h, w, s, g);
    println!("{}", if ans { "Yes" } else { "No" });
}
