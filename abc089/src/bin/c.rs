use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let chars = "MARCH".chars().collect::<Vec<char>>();
    let mut count = vec![0; 5];
    for s_i in s {
        if let Some(index) = chars.iter().position(|&c_i| c_i == s_i[0]) {
            count[index] += 1;
        }
    }
    let mut sum = 0_i64;
    for i in 0..5 - 2 {
        for j in i + 1..5 - 1 {
            for k in j + 1..5 {
                sum += count[i] * count[j] * count[k];
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}
