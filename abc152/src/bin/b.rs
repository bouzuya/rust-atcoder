use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = a.to_string().repeat(b).min(b.to_string().repeat(a));
    println!("{}", ans);
}
