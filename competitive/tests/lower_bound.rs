use competitive::lower_bound::*;

#[test]
fn test_lower_bound() {
    assert_eq!(0, lower_bound(&vec![1, 3, 3, 3, 5], &0));
    assert_eq!(0, lower_bound(&vec![1, 3, 3, 3, 5], &1));
    assert_eq!(1, lower_bound(&vec![1, 3, 3, 3, 5], &2));
    assert_eq!(1, lower_bound(&vec![1, 3, 3, 3, 5], &3));
    assert_eq!(4, lower_bound(&vec![1, 3, 3, 3, 5], &4));
    assert_eq!(4, lower_bound(&vec![1, 3, 3, 3, 5], &5));
    assert_eq!(5, lower_bound(&vec![1, 3, 3, 3, 5], &6));
}

#[test]
fn test_lower_bound_by() {
    assert_eq!(0, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&0)));
    assert_eq!(0, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&1)));
    assert_eq!(1, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&2)));
    assert_eq!(1, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&3)));
    assert_eq!(4, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&4)));
    assert_eq!(4, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&5)));
    assert_eq!(5, lower_bound_by(&vec![1, 3, 3, 3, 5], |i| i.cmp(&6)));
}

#[test]
fn test_lower_bound_by_key() {
    assert_eq!(0, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &0, |&i| i));
    assert_eq!(0, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &1, |&i| i));
    assert_eq!(1, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &2, |&i| i));
    assert_eq!(1, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &3, |&i| i));
    assert_eq!(4, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &4, |&i| i));
    assert_eq!(4, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &5, |&i| i));
    assert_eq!(5, lower_bound_by_key(&vec![1, 3, 3, 3, 5], &6, |&i| i));
}
