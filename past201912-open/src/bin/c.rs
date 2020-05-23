use proconio::input;

fn main() {
    input! {
        mut abcdef: [i64; 6],
    };
    abcdef.sort_by_key(|x| -x);
    let ans = abcdef[2];
    println!("{}", ans);
}
