use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    };
    let mut count = 0;
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let d = b[y][x] - a[y][x];
            a[y][x] += d;
            a[y + 1][x] += d;
            a[y][x + 1] += d;
            a[y + 1][x + 1] += d;
            count += d.abs();

            if (x + 1 == w - 1 && b[y][x + 1] != a[y][x + 1])
                || (y + 1 == h - 1 && b[y + 1][x] != a[y + 1][x])
            {
                println!("No");
                return;
            }
        }
    }
    let ans = count;
    println!("Yes");
    println!("{}", ans);
}
