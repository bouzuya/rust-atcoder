use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut max = 1;
    for x in 1.. {
        if x * x * x > n {
            break;
        }

        let cs = (x * x * x).to_string().chars().collect::<Vec<char>>();
        let mut ok = true;
        for i in 0..cs.len() / 2 {
            if cs[i] != cs[cs.len() - 1 - i] {
                ok = false;
                break;
            }
        }
        if ok {
            max = x * x * x;
        }
    }
    let ans = max;
    println!("{}", ans);
}
