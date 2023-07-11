use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        st: [(Chars, usize); n],
    };

    let mut ans = vec![];
    for x in 0..=9999 {
        let x = format!("{:04}", x).chars().collect::<Vec<char>>();

        let mut ok = true;
        for (s, t) in st.iter() {
            let count = x
                .iter()
                .copied()
                .zip(s.iter().copied())
                .filter(|(x_i, s_i)| x_i == s_i)
                .count();
            match t {
                1 => {
                    if count != 4 {
                        ok = false;
                    }
                }
                2 => {
                    if count != 3 {
                        ok = false;
                    }
                }
                3 => {
                    if count >= 3 {
                        ok = false;
                    }
                }
                _ => unreachable!(),
            }
        }

        if ok {
            ans.push(x);
        }
    }

    if ans.len() != 1 {
        println!("Can't Solve");
        return;
    }

    println!("{}", ans[0].iter().collect::<String>());
}
