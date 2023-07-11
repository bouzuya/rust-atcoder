use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut count = 0_usize;
    for x in a..=b {
        let cs = x.to_string().chars().collect::<Vec<char>>();
        let mut ok = true;
        for i in 0..cs.len() / 2 {
            if cs[i] != cs[cs.len() - 1 - i] {
                ok = false;
                break;
            }
        }
        if ok {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
