use self::mod_u32::ModU32;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    // a,b,c,? -> 0,1,2,3
    let c = s
        .iter()
        .map(|&c_i| match c_i {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            '?' => 3,
            _ => unreachable!(),
        })
        .collect::<Vec<usize>>();
    // cc[i][j]: i 番目までの j の個数
    let mut cc = vec![vec![0; 4]; n + 1];
    for (i, &c_i) in c.iter().enumerate() {
        for j in 0..4 {
            cc[i + 1][j] = cc[i][j];
        }
        cc[i + 1][c_i] += 1;
    }
    let mut count = ModU32::new(0);
    for i_b in 1..=n - 1 {
        if c[i_b] != 1 && c[i_b] != 3 {
            continue;
        }
        let c_q = cc[n][3] as i64 - if c[i_b] == 3 { 1 } else { 0 };
        let c_ql = cc[i_b][3];
        let c_qr = cc[n][3] - cc[i_b + 1][3];
        let c_a = cc[i_b][0];
        let c_c = cc[n][2] - cc[i_b + 1][2];
        // a+c
        count += ModU32::new(c_a) * ModU32::new(c_c) * ModU32::new(3).pow(c_q as u32);
        // a+?
        count += ModU32::new(c_a)
            * ModU32::new(c_qr)
            * ModU32::new(3).pow(std::cmp::max(0, c_q - 1) as u32);
        // ?+c
        count += ModU32::new(c_ql)
            * ModU32::new(c_c)
            * ModU32::new(3).pow(std::cmp::max(0, c_q - 1) as u32);
        // ?+?
        count += ModU32::new(c_ql)
            * ModU32::new(c_qr)
            * ModU32::new(3).pow(std::cmp::max(0, c_q - 2) as u32);
    }
    let ans: u32 = count.into();
    println!("{}", ans);
}

mod mod_u32 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ModU32(u32);

    const MODP: u32 = 1_000_000_007;

    impl ModU32 {
        pub fn inv(self) -> Self {
            if self.0 == 0 {
                panic!()
            };
            self.pow(MODP - 2)
        }

        pub fn new(x: u64) -> Self {
            Self((x % (MODP as u64)) as u32)
        }

        pub fn pow(self, exp: u32) -> Self {
            let mut b: ModU32 = self;
            let mut a: ModU32 = Self::new(1);
            let mut x = exp;
            while x > 1 {
                if x & 1 == 1 {
                    a *= b;
                }
                x /= 2;
                b *= b;
            }
            if x == 1 {
                a *= b;
            }
            a
        }
    }

    impl From<u32> for ModU32 {
        fn from(x: u32) -> Self {
            ModU32::new(x as u64)
        }
    }

    impl From<u64> for ModU32 {
        fn from(x: u64) -> Self {
            ModU32::new(x)
        }
    }

    impl From<usize> for ModU32 {
        fn from(x: usize) -> Self {
            ModU32::from(x as u64)
        }
    }

    impl Into<u32> for ModU32 {
        fn into(self) -> u32 {
            self.0
        }
    }

    impl Into<u64> for ModU32 {
        fn into(self) -> u64 {
            self.0 as u64
        }
    }

    impl Into<usize> for ModU32 {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl std::fmt::Display for ModU32 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::ops::Add<ModU32> for ModU32 {
        type Output = ModU32;

        fn add(self, rhs: ModU32) -> Self::Output {
            (self.0 as u64 + rhs.0 as u64).into()
        }
    }

    impl std::ops::AddAssign<ModU32> for ModU32 {
        fn add_assign(&mut self, rhs: ModU32) {
            self.0 = (*self + rhs).0
        }
    }

    impl std::ops::Sub<ModU32> for ModU32 {
        type Output = ModU32;

        fn sub(self, rhs: ModU32) -> Self::Output {
            (if self.0 < rhs.0 {
                self.0 + (MODP - rhs.0)
            } else {
                self.0 - rhs.0
            })
            .into()
        }
    }

    impl std::ops::SubAssign<ModU32> for ModU32 {
        fn sub_assign(&mut self, rhs: ModU32) {
            self.0 = (*self - rhs).0
        }
    }

    impl std::ops::Mul<ModU32> for ModU32 {
        type Output = ModU32;

        fn mul(self, rhs: ModU32) -> Self::Output {
            Self::new((self.0 as u64) * (rhs.0 as u64))
        }
    }

    impl std::ops::MulAssign<ModU32> for ModU32 {
        fn mul_assign(&mut self, rhs: ModU32) {
            self.0 = (*self * rhs).0
        }
    }

    impl std::ops::Div<ModU32> for ModU32 {
        type Output = ModU32;

        fn div(self, rhs: ModU32) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl std::ops::DivAssign<ModU32> for ModU32 {
        fn div_assign(&mut self, rhs: ModU32) {
            self.0 = (*self / rhs).0
        }
    }
}
