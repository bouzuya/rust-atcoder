use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64,
    };
    let mut a = x;
    let mut l = 1;
    loop {
        a *= 2;
        if a > y {
            break;
        }
        l += 1;
    }
    let ans = l;
    println!("{}", ans);
}
