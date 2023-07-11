use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    };

    let mut count = 0_usize;
    for i in 1..n {
        let c = a[i - 1] + a[i];
        if c <= x {
            continue;
        } else {
            let d = c - x;
            if d <= a[i] {
                a[i] -= d;
            } else {
                let d2 = d - a[i];
                a[i] = 0;
                a[i - 1] -= d2;
            }
            count += d;
        }
    }
    let ans = count;
    println!("{}", ans);
}
