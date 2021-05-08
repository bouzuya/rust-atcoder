use proconio::input;

fn f(a: &Vec<usize>, count: &Vec<Vec<usize>>, end_i: usize, x: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut y = x;
    for i in (0..=end_i).rev() {
        if count[i][y] > 0 {
            if y == a[i] {
                res.push(i);
                break;
            } else if i > 0 && count[i - 1][(200 + y - a[i]) % 200] > 0 {
                res.push(i);
                y = (200 + y - a[i]) % 200;
            } else if i == 0 {
                res.push(0);
            } else {
                // unreachable!();
            }
        }
    }
    res.reverse();
    res
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let a = a.iter().map(|&a_i| a_i % 200).collect::<Vec<usize>>();
    let mut count = vec![vec![0; 200]; n + 1];
    for i in 0..n {
        let a_i = a[i];
        for j in i + 1..n {
            let a_j = a[j];
            if a_i == a_j {
                println!("Yes");
                println!("1 {}", i + 1);
                println!("1 {}", j + 1);
                return;
            }
        }
    }
    count[0][a[0]] += 1;
    for i in 0..n - 1 {
        let a_i = a[i + 1];
        for j in 0..200 {
            count[i + 1][j] += count[i][j];
        }
        for j in 0..200 {
            if j == 0 || count[i][j] > 0 {
                count[i + 1][(j + a_i) % 200] += 1;
                if count[i + 1][(j + a_i) % 200] > 1 {
                    println!("Yes");
                    let res_b = f(&a, &count, i, (j + a_i) % 200);
                    let res_c = f(&a, &count, i + 1, (j + a_i) % 200);
                    print!("{} ", res_b.len());
                    for (k, &b_k) in res_b.iter().enumerate() {
                        print!(
                            "{}{}",
                            b_k + 1,
                            if k == res_b.len() - 1 { "\n" } else { " " }
                        )
                    }
                    print!("{} ", res_c.len());
                    for (k, &c_k) in res_c.iter().enumerate() {
                        print!(
                            "{}{}",
                            c_k + 1,
                            if k == res_c.len() - 1 { "\n" } else { " " }
                        )
                    }
                    return;
                }
            }
        }
    }
    println!("No");
}
