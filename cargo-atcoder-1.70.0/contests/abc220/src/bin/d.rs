use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let p = 998_244_353_usize;
    let mut count = vec![0_usize; 10];
    count[a[0]] += 1;
    for a_i in a.iter().copied().skip(1) {
        let mut next = vec![0_usize; 10];
        for (i, c) in count.iter().copied().enumerate() {
            next[(i + a_i) % 10] += c;
            next[(i + a_i) % 10] %= p;
            next[(i * a_i) % 10] += c;
            next[(i * a_i) % 10] %= p;
        }
        count = next;
    }
    for ans in count {
        println!("{}", ans);
    }
}
