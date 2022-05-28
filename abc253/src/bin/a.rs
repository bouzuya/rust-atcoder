use proconio::input;

fn main() {
    input! {
        mut abc: [usize; 3]
    };
    let b = abc[1];
    abc.sort();
    let ans = abc[1] == b;
    println!("{}", if ans { "Yes" } else { "No" });
}
