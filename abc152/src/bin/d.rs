use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = vec![vec![0_usize; 10]; 10];
    for i in 1..=n {
        let chars = i.to_string().chars().collect::<Vec<char>>();
        let first = (*chars.first().unwrap() as u8 - b'0') as usize;
        let last = (*chars.last().unwrap() as u8 - b'0') as usize;
        count[first][last] += 1;
    }
    let mut ans = 0_usize;
    for i in 0..=9 {
        for j in 0..=9 {
            ans += count[i][j] * count[j][i];
        }
    }
    println!("{}", ans);
}
