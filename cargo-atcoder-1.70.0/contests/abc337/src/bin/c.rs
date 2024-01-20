use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut start = 0_usize;
    let mut next = vec![n; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i == -1 {
            start = i;
            continue;
        }
        next[(a_i - 1) as usize] = i;
    }

    let mut index = start;
    loop {
        println!("{}", index + 1);
        index = next[index];
        if index == n {
            break;
        }
    }
}
