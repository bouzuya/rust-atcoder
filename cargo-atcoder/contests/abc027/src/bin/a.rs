use proconio::input;

fn main() {
    input! {
        l: [usize; 3],
    };
    let ans = if l[0] == l[1] {
        l[2]
    } else if l[0] == l[2] {
        l[1]
    } else {
        l[0]
    };
    println!("{}", ans);
}
