use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0;
    for i in 1..=n {
        if i.to_string().chars().any(|c| c == '7') {
            continue;
        }
        let mut ok = true;
        let mut j = i;
        while j > 0 {
            let x = j % 8;
            if x == 7 {
                ok = false;
                break;
            }
            j /= 8;
        }
        if !ok {
            continue;
        }
        count += 1;
    }
    let ans = count;
    println!("{}", ans);
}
