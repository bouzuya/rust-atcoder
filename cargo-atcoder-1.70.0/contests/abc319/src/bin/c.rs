use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        c: [[usize; 3]; 3],
    };

    // 0, 1, 2
    // 3, 4, 5
    // 6, 7, 8
    let c = {
        let mut c2 = vec![0_usize; 9];
        for i in 0..3 {
            for j in 0..3 {
                c2[i * 3 + j] = c[i][j];
            }
        }
        c2
    };

    let lines = vec![
        // -
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        // |
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        // \
        vec![0, 4, 8],
        // /
        vec![2, 4, 6],
    ];

    let mut count = 0_f64;
    let mut ks = (0..9).collect::<Vec<usize>>();
    loop {
        let mut opened = vec![false; 9];
        let mut disappointed = false;
        for k in ks.iter().copied() {
            opened[k] = true;
            for line in lines.iter() {
                if !line.iter().copied().all(|x| opened[x]) {
                    continue;
                }
                let l1 = match line.iter().copied().position(|x| x == k) {
                    None => continue,
                    Some(index) => index,
                };
                let (l2, l3) = ((l1 + 1) % 3, (l1 + 2) % 3);
                if c[line[l2]] == c[line[l3]] && c[line[l2]] != c[line[l1]] {
                    disappointed = true;
                }
            }
        }
        if !disappointed {
            count += 1_f64;
        }
        if !ks.next_permutation() {
            break;
        }
    }
    let all = (1..=9).product::<usize>() as f64;
    let ans = count / all;
    println!("{}", ans);
}
