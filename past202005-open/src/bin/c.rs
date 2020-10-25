use proconio::input;

fn main() {
    input! {
        a: usize,
        r: usize,
        n: usize,
    };
    if r == 1 {
        println!("{}", a);
        return;
    }
    let mut ans = a;
    for _ in 1..n {
        match ans.checked_mul(r) {
            None => {
                println!("large");
                return;
            }
            Some(x) => {
                if x > 1_000_000_000_usize {
                    println!("large");
                    return;
                }
                ans = x;
            }
        }
    }
    println!("{}", ans);
}
