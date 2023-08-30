use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    if a.contains(&0) {
        println!("0");
        return;
    }

    let mut ans = 1_usize;
    for a_i in a {
        match ans.checked_mul(a_i) {
            Some(x) => {
                if x > 1_000_000_000_000_000_000_usize {
                    println!("-1");
                    return;
                }
                ans = x;
            }
            None => {
                println!("-1");
                return;
            }
        }
    }
    println!("{}", ans);
}
