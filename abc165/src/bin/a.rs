use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    };
    // b / k * k は b 以下で最大の k の倍数
    let ans = if a <= b / k * k { "OK" } else { "NG" };
    println!("{}", ans);
}
