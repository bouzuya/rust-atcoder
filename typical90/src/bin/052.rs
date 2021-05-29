use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u64; 6]; n],
    };

    let p = 1_000_000_007;
    let mut x = 1_u64;
    for a_i in a {
        let sum = a_i.iter().sum::<u64>();
        x *= sum;
        x %= p;
    }
    let ans = x;
    println!("{}", ans);
}
