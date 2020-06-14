use proconio::input;

fn main() {
    input! {
        x: [i64; 5],
    };
    let ans = x.iter().position(|&x_i| x_i == 0).unwrap() + 1;
    println!("{}", ans);
}
