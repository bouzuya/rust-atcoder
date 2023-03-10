use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = vec![vec![0_usize; 10]; 10];
    for x in 1..=n {
        let s = x.to_string().chars().collect::<Vec<char>>();
        let h = (*s.first().unwrap() as u8 - b'0') as usize;
        let t = (*s.last().unwrap() as u8 - b'0') as usize;
        count[h][t] += 1;
    }
    let mut ans = 0_usize;
    for h in 1..10 {
        for t in 1..10 {
            ans += count[h][t] * count[t][h];
        }
    }
    println!("{}", ans);
}
