use proconio::input;

fn main() {
    input! {
        (a, b, c): (i64, i64, i64)
    };
    let mut vs = vec![a * b, a * c, b * c];
    vs.sort();
    println!("{} {}", vs[0], vs[2]);
}
