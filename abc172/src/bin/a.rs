use proconio::input;

fn main() {
    input! {
        a: usize
    };
    let ans = a + a.pow(2) + a.pow(3);
    println!("{}", ans);
}
