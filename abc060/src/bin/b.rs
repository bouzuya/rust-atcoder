use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    };
    for x in 1..b {
        if (x * a) % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
