use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    };
    let s = "wbwbwwbwbwbw".repeat(100).chars().collect::<Vec<char>>();
    for i in 0.."wbwbwwbwbwbw".len() {
        let mut count_w = 0_usize;
        let mut count_b = 0_usize;
        for j in i..i + w + b {
            if s[j] == 'w' {
                count_w += 1;
            } else {
                count_b += 1;
            }
        }
        if count_w == w && count_b == b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
