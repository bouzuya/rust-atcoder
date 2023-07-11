use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let n = format!("{}{}", a, b).parse::<usize>().unwrap();
    let mut ok = false;
    for i in 1.. {
        if i * i > n {
            break;
        }
        if i * i == n {
            ok = true;
            break;
        }
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}
