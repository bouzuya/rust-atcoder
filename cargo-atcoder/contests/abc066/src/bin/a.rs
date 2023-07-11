use proconio::input;

fn main() {
    input! {
        abc: [i64; 3],
    };
    let ans = abc.iter().sum::<i64>() - abc.iter().max().unwrap();
    println!("{}", ans);
}
