use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let s_y = s[0..4].iter().collect::<String>().parse::<usize>().unwrap();
    let s_m = s[5..7].iter().collect::<String>().parse::<usize>().unwrap();
    let s_d = s[8..10]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let t_y = t[0..4].iter().collect::<String>().parse::<usize>().unwrap();
    let t_m = t[5..7].iter().collect::<String>().parse::<usize>().unwrap();
    let t_d = t[8..10]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut count = 0_usize;
    let mut weekday = 5;
    let mut is_in = false;
    for y in 2022..=9999 {
        let is_leap_year = (y % 400 == 0) || (y % 4 == 0 && y % 100 != 0);
        for m in 1..=12 {
            let days_of_month = vec![
                0,
                31,
                if is_leap_year { 29 } else { 28 },
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];
            for d in 1..=days_of_month[m] {
                if y == s_y && m == s_m && d == s_d {
                    is_in = true;
                }
                if is_in && (weekday == 5 || weekday == 6) {
                    count += 1;
                }

                if y == t_y && m == t_m && d == t_d {
                    is_in = false;
                }

                weekday += 1;
                weekday %= 7;
            }
        }
    }
    println!("{}", count);
}
