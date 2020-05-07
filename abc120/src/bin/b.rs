use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };
    let mut ds = vec![];
    for i in 1..=std::cmp::min(a, b) {
        if a % i == 0 && b % i == 0 {
            ds.push(i);
        }
    }
    ds.reverse();
    println!("{}", ds[k - 1]);
}
