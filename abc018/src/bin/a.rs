use proconio::input;

fn main() {
    input! {
        abc: [i64; 3],
    };
    let mut sorted = abc.clone();
    sorted.sort();
    sorted.reverse();
    for v in abc {
        println!("{}", sorted.iter().position(|&x| x == v).unwrap() + 1);
    }
}
