use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    if s[0] == '1' {
        println!("No");
        return;
    }

    let columns = vec![
        vec![6],
        vec![3],
        vec![1, 7],
        vec![0, 4],
        vec![2, 8],
        vec![5],
        vec![9],
    ];
    let mut c = vec![];
    for col in columns {
        let mut c_i = vec![];
        for i in col {
            c_i.push(s[i] == '1');
        }
        c.push(c_i);
    }

    for i in 0..c.len() {
        if !c[i].iter().any(|&b| b) {
            continue;
        }
        for j in i + 1..c.len() {
            if !c[j].iter().all(|&b| !b) {
                continue;
            }
            for k in j + 1..c.len() {
                if c[k].iter().any(|&b| b) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
