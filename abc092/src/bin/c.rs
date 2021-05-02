use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut sum = 0;
    let mut p = 0;
    for &a_i in a.iter() {
        sum += (p - a_i).abs();
        p = a_i;
    }
    sum += (p - 0).abs();
    for i in 0..n {
        let a_p = if i == 0 { 0 } else { a[i - 1] };
        let a_i = a[i];
        let a_n = if i == n - 1 { 0 } else { a[i + 1] };
        println!(
            "{}",
            sum - (a_p - a_i).abs() - (a_i - a_n).abs() + (a_p - a_n).abs()
        );
    }
}
