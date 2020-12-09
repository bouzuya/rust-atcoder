use proconio::input;
use proconio::marker::Chars;
use superslice::*;

fn is_ok(s: &Vec<char>, t: &Vec<char>) -> bool {
    let sorted_s = {
        let mut s = s.clone();
        s.sort();
        s
    };
    let sorted_t = {
        let mut s = t.clone();
        s.sort();
        s
    };
    if sorted_s != sorted_t {
        return false;
    }
    if s == t {
        return false;
    }
    let reversed_t = {
        let mut s = t.clone();
        s.reverse();
        s
    };
    if s == &reversed_t {
        return false;
    }
    true
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut is = (0..n).collect::<Vec<usize>>();
    loop {
        let t = is.iter().map(|&i| s[i]).collect::<Vec<char>>();
        if is_ok(&s, &t) {
            println!("{}", t.into_iter().collect::<String>());
            return;
        }

        if !is.next_permutation() {
            break;
        }
    }
    println!("None");
}
