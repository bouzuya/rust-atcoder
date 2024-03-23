use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = vec![];
    for i in 0..n - 1 {
        b.push(a[i] * a[i + 1]);
    }
    for b_i in b {
        println!("{}", b_i);
    }
}
