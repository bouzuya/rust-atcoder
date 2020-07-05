use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut ans = vec![0; 4];
    for s_i in s.iter() {
        match s_i.as_str() {
            "AC" => ans[0] += 1,
            "WA" => ans[1] += 1,
            "TLE" => ans[2] += 1,
            "RE" => ans[3] += 1,
            _ => unreachable!(),
        }
    }
    println!("AC x {}", ans[0]);
    println!("WA x {}", ans[1]);
    println!("TLE x {}", ans[2]);
    println!("RE x {}", ans[3]);
}
