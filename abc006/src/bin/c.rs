use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    if m < n * 2 || n * 4 < m {
        println!("{} {} {}", -1, -1, -1);
        return;
    }
    let mut n_a = n;
    let mut n_b = 0;
    let mut n_c = 0;
    let x = m - (n_a * 2 + n_b * 3 + n_c * 4);
    n_b = x % 2;
    n_c = x / 2;
    n_a = n_a - n_b - n_c;
    println!("{} {} {}", n_a, n_b, n_c);
}
