use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n <= 125 {
        4
    } else if n <= 211 {
        6
    } else if n <= 214 {
        8
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
