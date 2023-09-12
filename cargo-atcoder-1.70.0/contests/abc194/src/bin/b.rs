use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut min = std::usize::MAX;
    for (i, (a, _)) in ab.iter().copied().enumerate() {
        for (j, (_, b)) in ab.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            min = min.min(a.max(b));
        }
    }
    for (a, b) in ab {
        min = min.min(a + b);
    }
    let ans = min;
    println!("{ans}");
}
