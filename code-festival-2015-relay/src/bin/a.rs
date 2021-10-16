use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: Usize1,
    };
    let mut teams = vec![vec![]; 20];
    let mut i = 0;
    let mut ltr = true;
    for rank in 1..=200 {
        teams[i].push(rank);
        if rank % 20 == 0 {
            ltr = !ltr;
        } else {
            assert!(rank % 20 != 0);
            if ltr {
                i += 1;
            } else {
                i -= 1;
            }
        }
    }
    let ans = teams[t].iter().sum::<usize>();
    println!("{}", ans);
}
