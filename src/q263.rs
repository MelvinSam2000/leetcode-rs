/*
    263 - Ugly Number
    Time: O(logn)
    Space: O(1)
*/
pub fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let factors = [2, 3, 5];
    let mut done = false;
    while !done {
        done = true;
        for factor in factors {
            if n % factor == 0 {
                n /= factor;
                done = false;
            }
        }
    }
    n == 1
}

/*
    263 - Ugly Number (ebic version)
    Time: O(1)
    Space: O(1)
    Note: Wont pass leetcode cuz of large number testcases
*/
pub fn is_ugly_ebic(n: i32) -> bool {
    const N: usize = 10000;
    const RECURSION_LIMIT: usize = 1000;
    static UGLY: [bool; N] = {
        let mut ugly = [false; N];
        let factors = [2, 3, 5];
        let mut i = 0;
        while i < N {
            let mut j = i;
            let mut done = false;
            let mut l = 0;
            while l < RECURSION_LIMIT && !done {
                done = true;
                let mut k = 0;
                while k < 3 {
                    if j % factors[k] == 0 {
                        j /= factors[k];
                        done = false;
                    }
                    k += 1;
                }
                l += 1;
            }
            ugly[i] = j == 1;
            i += 1;
        }
        ugly
    };

    if n <= 0 {
        return false;
    }
    UGLY[n as usize]
}
