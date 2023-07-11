use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    if n % 2 != 0 {
        println!("error");
        return;
    }
    let sum = a.windows(2).step_by(2).map(|p| p[1] - p[0]).sum::<usize>();
    let ans = sum;
    println!("{}", ans);
}
