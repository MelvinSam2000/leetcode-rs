/*
    875 - Koko eating bananas
    Time: O(n*log(max(piles)))
    Space: O(1)
*/
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let h = h as u32;
    let (mut l, mut r) =
        (1, piles.iter().max().cloned().unwrap());
    let mut res = r;
    while l <= r {
        let k = (l + r) / 2;
        let time = piles
            .iter()
            .map(|&pile| {
                (pile % k != 0) as u32 + (pile / k) as u32
            })
            .sum::<u32>();
        if time <= h {
            r = k - 1;
            res = res.min(k);
        } else {
            l = k + 1;
        }
    }
    res
}
