use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    };
    let h = q - p + 1;
    let w = s - r + 1;
    let mut ans = vec![vec!['.'; w as usize]; h as usize];
    for i in 0..h {
        let y = i + p;
        for j in 0..w {
            let x = j + r;
            if y + x == a + b || y - x == a - b {
                ans[i as usize][j as usize] = '#';
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", ans[i as usize][j as usize]);
        }
        println!();
    }
}
