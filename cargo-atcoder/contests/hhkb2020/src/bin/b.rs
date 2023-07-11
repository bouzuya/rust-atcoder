use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut sum = 0;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '#' {
                continue;
            }
            let dir = vec![(0, 1), (1, 0)];
            for (d_h, d_w) in dir {
                let r_next = r as i64 + d_h;
                let c_next = c as i64 + d_w;
                if (0..h as i64).contains(&r_next) && (0..w as i64).contains(&c_next) {
                    let r_next = r_next as usize;
                    let c_next = c_next as usize;
                    if s[r_next][c_next] == '#' {
                        continue;
                    }
                    sum += 1;
                }
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}
