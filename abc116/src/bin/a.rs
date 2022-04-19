use proconio::input;

fn main() {
    input! {
        ab: usize,
        bc: usize,
        _ca: usize,
    };
    let ans = ab * bc / 2;
    println!("{}", ans);
}
