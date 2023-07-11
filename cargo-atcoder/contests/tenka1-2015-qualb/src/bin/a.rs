fn main() {
    let max_n = 20 - 1;
    let mut a = vec![0_i64; max_n + 1];
    a[0] = 100;
    a[1] = 100;
    a[2] = 200;
    for n in 3..=max_n {
        a[n] = a[n - 1] + a[n - 2] + a[n - 3];
    }

    let ans = a[max_n];
    println!("{}", ans);
}
