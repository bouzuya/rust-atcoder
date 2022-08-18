use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        mut abc: [usize; 3],
    };
    abc.sort();

    let mut max = 0;
    loop {
        max = max
            .max(abc[0] * 10 + abc[1] + abc[2])
            .max(abc[0] + abc[1] * 10 + abc[2]);
        if !abc.next_permutation() {
            break;
        }
    }
    let ans = max;
    println!("{}", ans);
}
