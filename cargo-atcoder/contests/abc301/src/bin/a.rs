use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let count_t = s.iter().copied().filter(|c| c == &'T').count();
    let count_a = n - count_t;
    let ans = if count_t > count_a {
        'T'
    } else if count_t < count_a {
        'A'
    } else {
        let mut count = count_t;
        let mut count_t = 0_usize;
        let mut count_a = 0_usize;
        let mut c = ' ';
        for s_i in s {
            if s_i == 'T' {
                count_t += 1;
                if count_t == count {
                    c = 'T';
                    break;
                }
            } else {
                count_a += 1;
                if count_a == count {
                    c = 'A';
                    break;
                }
            }
        }
        c
    };
    println!("{}", ans);
}
