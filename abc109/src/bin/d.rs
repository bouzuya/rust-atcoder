use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[u8; w]; h],
    };
    let mut o = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] % 2 != 0 {
                if j + 1 < w && a[i][j + 1] % 2 != 0 {
                    a[i][j] -= 1;
                    a[i][j + 1] += 1;
                    o.push((i, j, i, j + 1));
                } else if i + 1 < h && a[i + 1][j] % 2 != 0 {
                    a[i][j] -= 1;
                    a[i + 1][j] += 1;
                    o.push((i, j, i + 1, j));
                } else {
                    if j + 1 < w {
                        a[i][j] -= 1;
                        a[i][j + 1] += 1;
                        o.push((i, j, i, j + 1));
                    } else if i + 1 < h {
                        a[i][j] -= 1;
                        a[i + 1][j] += 1;
                        o.push((i, j, i + 1, j));
                    }
                }
            }
        }
    }
    println!("{}", o.len());
    for (y_i, x_i, y_j, x_j) in o.iter() {
        println!("{} {} {} {}", y_i + 1, x_i + 1, y_j + 1, x_j + 1);
    }
}
