use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let floors = [
        "B9", "B8", "B7", "B6", "B5", "B4", "B3", "B2", "B1", "1F", "2F", "3F", "4F", "5F", "6F",
        "7F", "8F", "9F",
    ];
    let f1 = floors.iter().position(|&f| f == s).unwrap();
    let f2 = floors.iter().position(|&f| f == t).unwrap();
    let ans = f1.max(f2) - f1.min(f2);
    println!("{}", ans);
}
