use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let k = {
        let mut k = 1;
        loop {
            if k * (k - 1) == 2 * n {
                break k;
            }
            if k * (k - 1) > 2 * n {
                println!("No");
                return;
            }
            k += 1;
        }
    };

    let mut x = 1;
    let mut s = vec![vec![]; k];
    for i in 0..k - 1 {
        for j in i + 1..k {
            s[i].push(x);
            s[j].push(x);
            x += 1;
        }
    }

    println!("Yes");
    println!("{}", k);
    for s_i in s {
        print!("{} ", s_i.len());
        for (j, &s_ij) in s_i.iter().enumerate() {
            print!("{}{}", s_ij, if j == s_i.len() - 1 { "\n" } else { " " });
        }
    }
}
