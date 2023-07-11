use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    };
    let mut count = 1;
    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for &(dx, dy) in dir.iter() {
        let mut i = x as i64;
        let mut j = y as i64;
        loop {
            i += dx;
            j += dy;
            if !(0..h as i64).contains(&i) || !(0..w as i64).contains(&j) {
                break;
            }
            let (i, j) = (i as usize, j as usize);
            if s[i][j] == '#' {
                break;
            }
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
