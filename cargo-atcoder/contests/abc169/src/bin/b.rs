use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    if a.contains(&0) {
        println!("0");
        return;
    }
    let mut ans = 1_u64;
    for &a_i in a.iter() {
        if a_i > 1_000_000_000_000_000_000_u64 / ans {
            println!("-1");
            return;
        }
        ans = ans * a_i;
    }
    println!("{}", ans);
}
