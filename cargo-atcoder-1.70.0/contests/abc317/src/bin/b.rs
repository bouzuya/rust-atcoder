use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    let mut prev = a[0];
    for a_i in a.iter().copied().skip(1) {
        if prev + 1 != a_i {
            println!("{}", prev + 1);
            return;
        }
        prev = a_i;
    }
}
