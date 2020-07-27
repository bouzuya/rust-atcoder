use proconio::input;

fn main() {
    input! {
        d: i64,
    };
    let ans = match d {
        25 => "",
        24 => " Eve",
        23 => " Eve Eve",
        22 => " Eve Eve Eve",
        _ => unreachable!(),
    };
    println!("Christmas{}", ans);
}
