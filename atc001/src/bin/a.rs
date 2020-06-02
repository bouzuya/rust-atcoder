use proconio::input;
use proconio::marker::Chars;

fn dfs(
    used: &mut Vec<Vec<bool>>,
    h: usize,
    w: usize,
    s: (usize, usize),
    g: (usize, usize),
) -> bool {
    let d = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut stack = vec![];
    stack.push(s);
    while let Some((x, y)) = stack.pop() {
        if (x, y) == g {
            return true;
        }
        if used[y][x] {
            continue;
        }
        used[y][x] = true;
        for &(d_x, d_y) in d.iter() {
            let (x_n, y_n) = (x as i64 + d_x, y as i64 + d_y);
            if (0..h as i64).contains(&y_n) && (0..w as i64).contains(&x_n) {
                let (x_n, y_n) = (x_n as usize, y_n as usize); // shadowing
                stack.push((x_n, y_n));
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
    let ans = dfs(&mut used, h, w, s, g);
    println!("{}", if ans { "Yes" } else { "No" });
}
