use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();

    let x = a[n - 1];
    let mut k = (a[0], (x - 2 * a[0]).abs());
    for a_i in a.into_iter().take(n - 1) {
        let d = (x - 2 * a_i).abs();
        if d <= k.1 {
            k = (a_i, d);
        }
    }
    println!("{} {}", x, k.0);
}
