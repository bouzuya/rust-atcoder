use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut ans = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            let mut count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (ny, nx) = (i as i64 + dy, j as i64 + dx);
                    if !(0..n as i64).contains(&ny) || !(0..m as i64).contains(&nx) {
                        continue;
                    }
                    let (ny, nx) = (ny as usize, nx as usize);
                    if s[ny][nx] == '#' {
                        count += 1;
                    }
                }
            }
            ans[i][j] = count;
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
