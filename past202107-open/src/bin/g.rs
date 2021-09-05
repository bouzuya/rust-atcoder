use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let mut t = vec![];
    let mut sum = 0;
    for i in 0.. {
        let v = 3_i64.pow(i);
        sum += v;
        t.push((sum, v));
        if sum >= n {
            break;
        }
    }
    let mut ans = vec![];
    let mut x = n;
    while x != 0 {
        for (t_i, v) in t.iter().copied() {
            if t_i >= x.abs() {
                let v = v * x.signum();
                x -= v;
                ans.push(v);
                break;
            }
        }
    }

    ans.sort();
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
