use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };
    let mut count = 0_usize;
    for s_i in s {
        for t_i in t.iter() {
            if s_i.ends_with(t_i) {
                count += 1;
                break;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
