use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        mut c: i64,
    };
    let mut count = 0_i64;
    let mut p_a = None;
    let mut p_b = None;
    let mut p_c = None;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        if p_a == Some(a) && p_b == Some(b) && p_c == (Some(c)) {
            println!("-1");
            return;
        }
        p_a = Some(a);
        p_b = Some(b);
        p_c = Some(c);
        count += 1;

        let h_a = a / 2;
        let h_b = b / 2;
        let h_c = c / 2;
        a = h_b + h_c;
        b = h_a + h_c;
        c = h_a + h_b;
    }
    let ans = count;
    println!("{}", ans);
}
