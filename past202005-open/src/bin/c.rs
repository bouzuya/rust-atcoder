use proconio::input;

fn main() {
    input! {
        a: u64,
        r: u64,
        n: u32,
    };
    if r == 1 {
        println!("{}", a);
        return;
    }
    if n > 1_000_000_000_u64.next_power_of_two().trailing_zeros() {
        println!("large");
        return;
    }
    let ans = a * r.pow(n - 1);
    if ans > 1_000_000_000_u64 {
        println!("large");
        return;
    }
    println!("{}", ans);
}
