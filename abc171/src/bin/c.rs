use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let (l, m) = {
        let mut l = 0;
        let mut m = 0;
        while m < n {
            l += 1;
            m += 26_i64.pow(l);
        }
        (l, m - 26_i64.pow(l))
    };
    let ans = {
        let mut n = n - m - 1; // shadowing
        let mut s = vec![];
        for _ in 0..l {
            s.push(((n % 26) as u8 + b'a') as char);
            n /= 26;
        }
        s.iter().rev().collect::<String>()
    };
    println!("{}", ans);

    // let mut s = vec![];
    // while n > 0 {
    //     n -= 1;
    //     s.push(((n % 26) as u8 + b'a') as char);
    //     n /= 26;
    // }
    // println!("{}", s.iter().rev().collect::<String>());
}
