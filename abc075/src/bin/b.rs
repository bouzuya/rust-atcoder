use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut ans = vec![vec![0_i64; w]; h];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                ans[y][x] = -1;
            } else {
                let mut c = 0;
                let d = vec![-1, 0, 1];
                for &dy_i in d.iter() {
                    for &dx_i in d.iter() {
                        let (nx, ny) = (x as i64 + dx_i, y as i64 + dy_i);
                        if (0..w as i64).contains(&nx) && (0..h as i64).contains(&ny) {
                            let (nx, ny) = (nx as usize, ny as usize);
                            c += if s[ny][nx] == '#' { 1 } else { 0 };
                        }
                    }
                }
                ans[y][x] = c;
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            if ans[y][x] == -1 {
                print!("#");
            } else {
                print!("{}", ans[y][x]);
            }
        }
        println!();
    }
}
