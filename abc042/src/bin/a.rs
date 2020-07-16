use proconio::input;

fn main() {
    input! {
        mut abc: [i64; 3],
    };
    abc.sort();
    let ans = abc == vec![5, 5, 7];
    println!("{}", if ans { "YES" } else { "NO" });
}
