use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let ans = if x == y {
        x
    } else {
        let hands = vec![0, 1, 2];
        hands
            .iter()
            .filter(|&&h| h != x && h != y)
            .cloned()
            .collect::<Vec<usize>>()[0]
    };
    println!("{}", ans);
}
