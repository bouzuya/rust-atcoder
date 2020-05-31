use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u128; n],
    };
    if a.contains(&0) {
        println!("{}", 0);
    } else {
        let mut ans = 1_u128;
        for &a_i in a.iter() {
            ans = ans * a_i;
            if ans > 1_000_000_000_000_000_000_u128 {
                println!("-1");
                return;
            }
        }
        println!("{}", ans);
    }
}
