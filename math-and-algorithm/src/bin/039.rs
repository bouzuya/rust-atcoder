use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(usize, usize, i64); q],
    };

    let mut count = vec![0_i64; n + 1];
    for (l, r, x) in lrx {
        count[l - 1] += x;
        count[r] -= x;
    }
    for i in 0..n {
        count[i + 1] += count[i];
    }

    let mut ans = vec![];
    for i in (0..n).skip(1) {
        ans.push(
            match count[i - 1].cmp(&count[i]) {
                std::cmp::Ordering::Less => '<',
                std::cmp::Ordering::Equal => '=',
                std::cmp::Ordering::Greater => '>',
            }
            .to_string(),
        );
    }
    let ans = ans.join("");
    println!("{}", ans);
}
