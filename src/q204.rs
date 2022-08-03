/*
    204 - Count Primes
    Time: O(n)
    Space: O(n)
*/
pub fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    let mut is_prime = vec![true; n + 1];
    let mut count = 0;
    for i in 2..n {
        if is_prime[i] {
            count += 1;
            for j in (i..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    count
}
