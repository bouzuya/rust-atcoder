use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    let mut p = a[0];
    for a_i in a.into_iter().skip(1) {
        if a_i != p + 1 {
            println!("{}", p + 1);
            return;
        }
        p = a_i;
    }
}
