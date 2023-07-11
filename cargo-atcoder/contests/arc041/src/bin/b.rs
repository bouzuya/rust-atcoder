use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [Chars; n]
    };
    let mut c = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            c[i][j] = b[i][j] as u8 - b'0';
        }
    }

    let mut a = vec![vec![0; m]; n];
    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            let mut min_count = std::u8::MAX;
            for &(di, dj) in d.iter() {
                min_count = std::cmp::min(
                    min_count,
                    c[(i as i16 + di) as usize][(j as i16 + dj) as usize],
                );
            }
            for &(di, dj) in d.iter() {
                c[(i as i16 + di) as usize][(j as i16 + dj) as usize] -= min_count;
            }
            a[i][j] = min_count;
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", a[i][j]);
        }
        println!();
    }
}
