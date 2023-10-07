use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        xya: [(usize, usize, usize); n],
    };
    let mut xs = vec![false; w];
    let mut ys = vec![false; h];
    for (x, y, a) in xya {
        match a {
            1 => {
                for i in 0..x {
                    xs[i] = true;
                }
            }
            2 => {
                for i in x..w {
                    xs[i] = true;
                }
            }
            3 => {
                for i in 0..y {
                    ys[i] = true;
                }
            }
            4 => {
                for i in y..h {
                    ys[i] = true;
                }
            }
            _ => unreachable!(),
        }
    }
    let mut count = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if xs[j] || ys[i] {
                count += 1;
            }
        }
    }
    let ans = h * w - count;
    println!("{}", ans);
}
