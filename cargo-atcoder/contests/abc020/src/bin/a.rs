use proconio::input;

fn main() {
    input! {
        q: usize
    };
    let ans = match q {
        1 => "ABC",
        2 => "chokudai",
        _ => unreachable!(),
    };
    println!("{}", ans);
}
