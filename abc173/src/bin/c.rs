use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    };

    let mut ans = 0;
    for bits_y in 0..1 << h {
        for bits_x in 0..1 << w {
            let ys = (0..h)
                .map(|y| (bits_y >> y) & 1 == 1)
                .collect::<Vec<bool>>();
            let xs = (0..w)
                .map(|x| (bits_x >> x) & 1 == 1)
                .collect::<Vec<bool>>();
            let mut count = 0;
            for y in 0..h {
                for x in 0..w {
                    if c[y][x] == '#' && !(ys[y] || xs[x]) {
                        count += 1;
                    }
                }
            }
            if count == k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
