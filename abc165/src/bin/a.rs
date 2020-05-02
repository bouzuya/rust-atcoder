use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    };
    let mut ok = false;
    for i in a..=b {
        if i % k == 0 {
            ok = true;
        }
    }
    let ans = if ok { "OK" } else { "NG" };
    println!("{}", ans);
}
