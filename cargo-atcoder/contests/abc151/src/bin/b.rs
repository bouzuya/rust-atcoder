use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n - 1],
    };
    let sum = a.iter().sum::<usize>();
    for x in 0..=k {
        if sum + x >= m * n {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}
