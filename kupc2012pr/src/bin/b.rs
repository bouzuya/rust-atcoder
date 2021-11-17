use proconio::input;

fn main() {
    input! {
        n: usize,
        _k: usize,
        mut a: [String; n],
    };
    a.sort();
    a.reverse();
    let ans = a.into_iter().collect::<Vec<String>>().join("");
    println!("{}", ans);
}
