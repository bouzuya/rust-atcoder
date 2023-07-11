use proconio::input;

fn main() {
    input! {
        mut abc: [i64; 3],
    };
    abc.sort();
    let ans = abc[1];
    println!("{}", ans);
}
