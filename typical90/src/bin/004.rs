use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    };
    let mut s_h = vec![];
    for i in 0..h {
        let mut s = 0;
        for j in 0..w {
            s += a[i][j];
        }
        s_h.push(s);
    }
    let mut s_w = vec![];
    for j in 0..w {
        let mut s = 0;
        for i in 0..h {
            s += a[i][j];
        }
        s_w.push(s);
    }

    for i in 0..h {
        for j in 0..w {
            print!(
                "{}{}",
                s_h[i] + s_w[j] - a[i][j],
                if j == w - 1 { "\n" } else { " " }
            );
        }
    }
}
