use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut t = vec![(s[0], 1)];
    for c in s.into_iter().skip(1) {
        let (prev, len) = t.pop().unwrap();
        if prev == c {
            t.push((prev, len + 1));
        } else {
            t.push((prev, len));
            t.push((c, 1));
        }
    }

    let mut count = vec![0_usize; 26];
    for (c, len) in t {
        let index = (c as u8 - b'a') as usize;
        count[index] = count[index].max(len);
    }
    let ans = count.into_iter().sum::<usize>();
    println!("{}", ans);
}
