use proconio::input;

fn main() {
    input! {
        mut abcdef: [usize; 6],
    };
    abcdef.sort();
    abcdef.reverse();
    let ans = abcdef[2];
    println!("{}", ans);
}
