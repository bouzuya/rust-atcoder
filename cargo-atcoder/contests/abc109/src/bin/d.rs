use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[usize; w]; h],
    };
    for i in 0..h {
        for j in 0..w {
            a[i][j] %= 2;
        }
    }

    let mut ops = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                continue;
            }

            if j + 1 < w {
                a[i][j + 1] = if a[i][j + 1] == 1 { 0 } else { 1 };
                ops.push((i + 1, j + 1, i + 1, j + 2));
                continue;
            }

            if i + 1 < h {
                a[i + 1][j] = if a[i + 1][j] == 1 { 0 } else { 1 };
                ops.push((i + 1, j + 1, i + 2, j + 1));
                continue;
            }
        }
    }

    println!("{}", ops.len());
    for (y1, x1, y2, x2) in ops {
        println!("{} {} {} {}", y1, x1, y2, x2);
    }
}
