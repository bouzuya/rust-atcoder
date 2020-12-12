use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut p = a[0];
    for &a_i in a.iter().skip(1) {
        if p == a_i {
            println!("stay");
        } else if p > a_i {
            println!("down {}", p - a_i);
        } else if p < a_i {
            println!("up {}", a_i - p);
        } else {
            unreachable!();
        }
        p = a_i;
    }
}
