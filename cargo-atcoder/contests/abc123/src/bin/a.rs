use proconio::input;

fn main() {
    input! {
        abcde: [usize; 5],
        k: usize,
    };
    for (i, p) in abcde.iter().copied().enumerate() {
        for q in abcde.iter().copied().skip(i) {
            if q - p > k {
                println!(":(");
                return;
            }
        }
    }
    println!("Yay!");
}
