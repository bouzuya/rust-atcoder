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
                if y > 0 {
                    let ny = y - 1;
                    if x > 0 {
                        let nx = x - 1;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                    {
                        let nx = x;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                    if x < w - 1 {
                        let nx = x + 1;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                }
                {
                    let ny = y;
                    if x > 0 {
                        let nx = x - 1;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                    {
                        let nx = x;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                    if x < w - 1 {
                        let nx = x + 1;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                }
                if y < h - 1 {
                    let ny = y + 1;
                    if x > 0 {
                        let nx = x - 1;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                    {
                        let nx = x;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
                    }
                    if x < w - 1 {
                        let nx = x + 1;
                        c += if s[ny][nx] == '#' { 1 } else { 0 };
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
