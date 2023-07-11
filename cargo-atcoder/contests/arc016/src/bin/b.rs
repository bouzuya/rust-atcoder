use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        x: [Chars; n],
    };
    let mut p = vec!['.'; 10];
    let mut count = 0;
    for x_i in x {
        for (j, &x_ij) in x_i.iter().enumerate() {
            count += match x_ij {
                'x' => 1,
                'o' => {
                    if p[j] == x_ij {
                        0
                    } else {
                        1
                    }
                }
                '.' => 0,
                _ => unreachable!(),
            };
        }
        p = x_i;
    }

    let ans = count;
    println!("{}", ans);
}
