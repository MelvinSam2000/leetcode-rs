use std::collections::HashSet;

const ZERO_ASCII: u8 = 0x30;

/*
    202 - Happy Number
    Time - O(???)
    Space - O(???)
    Note: Time and Space complexity depend on how long it takes for the
    sequence a(n) to reach a cycle where:
        h(n) = squared sum of digits of n,
        a(n + 1) = h(a(n))
    Using python, it was verified that for n < 20000, it takes between 1 to 20,
    based on a uniform(?) random distribution.
*/
pub fn is_happy(n: i32) -> bool {
    let mut memo = HashSet::new();
    helper(n as usize, &mut memo)
}

fn helper(n: usize, memo: &mut HashSet<usize>) -> bool {
    let mut square = 0;
    for &digit in n.to_string().as_bytes() {
        let digit = digit - ZERO_ASCII;
        square += (digit as usize) * (digit as usize);
    }
    if memo.contains(&square) {
        return false;
    }
    if square == 1 {
        return true;
    }
    memo.insert(n);
    helper(square, memo)
}

#[test]
fn t1() {
    let tcases = [(4, false)];
    for (n, out) in tcases {
        assert_eq!(is_happy(n), out);
    }
}
