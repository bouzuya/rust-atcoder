use competitive::mod_u32::*;

#[test]
fn new() {
    assert_eq!(1u32, ModU32::new(1_000_000_008).into());
    assert_eq!(0u32, ModU32::new(1_000_000_007).into());
    assert_eq!(1000000006u32, ModU32::new(1_000_000_006).into());
    assert_eq!(1u32, ModU32::new(1).into());
    assert_eq!(0u32, ModU32::new(0).into());

    assert_eq!(ModU32::new(1), ModU32::new(1_000_000_012_000_000_036));
}

#[test]
fn inv() {
    fn test(n: i64) {
        assert_eq!(ModU32::from(n).inv() * ModU32::from(n), ModU32::new(1));
    }
    test(1_000_000_008);
    // test(1_000_000_007); // panic!
    test(1_000_000_006);
    test(1);
    // test(0); // panic!
    test(-1);
    test(-1_000_000_006);
    // test(-1_000_000_007); // panic!
    test(-1_000_000_008);
}

#[test]
fn pow() {
    assert_eq!(ModU32::new(1), ModU32::new(2).pow(0));
    assert_eq!(ModU32::new(2), ModU32::new(2).pow(1));
    assert_eq!(ModU32::new(4), ModU32::new(2).pow(2));
    assert_eq!(ModU32::new(536_870_912), ModU32::new(2).pow(29));
    assert_eq!(ModU32::new(1_073_741_824), ModU32::new(2).pow(30));
    assert_eq!(73_741_817i64, ModU32::new(1_073_741_824).into());
}

#[test]
fn from() {
    assert_eq!(ModU32::new(1_000_000_007), ModU32::from(1_000_000_007));
}

#[test]
fn into() {
    assert_eq!(0i64, ModU32::new(1_000_000_007).into());
}

#[test]
fn fmt() {
    assert_eq!("0", format!("{}", ModU32::new(1_000_000_007)));
}

#[test]
fn add() {
    assert_eq!(
        ModU32::new(1_000_000_007),
        ModU32::new(1_000_000_006) + ModU32::new(1)
    );
}

#[test]
fn add_assign() {
    let mut m = ModU32::new(1_000_000_006);
    m += ModU32::new(1);
    assert_eq!(ModU32::new(1_000_000_007), m);
}

#[test]
fn sub() {
    assert_eq!(
        ModU32::new(1_000_000_007),
        ModU32::new(1_000_000_008) - ModU32::new(1)
    );
    assert_eq!(ModU32::new(1_000_000_006), ModU32::new(0) - ModU32::new(1));
}

#[test]
fn sub_assign() {
    let mut m = ModU32::new(1_000_000_008);
    m -= ModU32::new(1);
    assert_eq!(ModU32::new(1_000_000_007), m);
}

#[test]
fn mul() {
    assert_eq!(ModU32::new(1), ModU32::new(1) * ModU32::new(1));
    assert_eq!(ModU32::new(6), ModU32::new(2) * ModU32::new(3));
    assert_eq!(
        ModU32::new(1_000_000_012_000_000_036),
        ModU32::new(1_000_000_006) * ModU32::new(1_000_000_006)
    );
    assert_eq!(
        ModU32::new(0),
        ModU32::new(1_000_000_007) * ModU32::new(1_000_000_007)
    );
}

#[test]
fn mul_assign() {
    let mut m = ModU32::new(2);
    m *= ModU32::new(3);
    assert_eq!(ModU32::new(6), m);
}

#[test]
fn div() {
    assert_eq!(ModU32::new(1), ModU32::new(1) / ModU32::new(1));
    assert_eq!(ModU32::new(0), ModU32::new(1_000_000_007) / ModU32::new(1));
    assert_eq!(
        ModU32::new(1),
        ModU32::new(1_000_000_006) / ModU32::new(1_000_000_006)
    );
}

#[test]
fn div_assign() {
    let mut m = ModU32::new(1_000_000_006);
    m /= ModU32::new(1_000_000_006);
    assert_eq!(ModU32::new(1), m);
}
