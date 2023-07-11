use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    };
    let mut sum = 0;
    for (i, &a_i) in a.iter().enumerate() {
        sum += a_i;
        if sum >= k {
            println!("{}", i + 1);
            return;
        }
    }
}
