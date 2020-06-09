use proconio::input;

fn main() {
    input! {
        a: u32,
        r: u32,
        n: u32,
    };
    if r == 1 {
        println!("{}", a);
        return;
    }
    if n > 1_000_000_000_u32.next_power_of_two().trailing_zeros() {
        println!("large");
        return;
    }
    let mut ans = a;
    for _ in 1..=n - 1 {
        ans *= r;
        if ans > 1_000_000_000_u32 {
            println!("large");
            return;
        }
    }
    println!("{}", ans);
}
