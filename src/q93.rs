/*
    93 - Restore IP addresses
    Time: O(1)
    Space: O(1)
*/
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let n = s.len();
    let mut res = vec![];
    let mut addr = [0, 0, 0, 0];
    for i in 1..=3 {
        if i >= n || (s[0] == b'0' && i != 1) {
            continue;
        }
        addr[0] = String::from_utf8_lossy(&s[..i])
            .parse::<i32>()
            .unwrap_or(256);
        if !(0..=255).contains(&addr[0]) {
            continue;
        }
        for j in i + 1..=i + 3 {
            if j >= n || (s[i] == b'0' && j != i + 1) {
                continue;
            }
            addr[1] = String::from_utf8_lossy(&s[i..j])
                .parse::<i32>()
                .unwrap_or(256);
            if !(0..=255).contains(&addr[1]) {
                continue;
            }
            for k in j + 1..=j + 3 {
                if k >= n || (s[j] == b'0' && k != j + 1) {
                    continue;
                }
                addr[2] = String::from_utf8_lossy(&s[j..k])
                    .parse::<i32>()
                    .unwrap_or(256);
                if !(0..=255).contains(&addr[2]) {
                    continue;
                }
                for l in k + 1..=k + 3 {
                    if l != n
                        || (s[k] == b'0' && l != k + 1)
                    {
                        continue;
                    }
                    addr[3] =
                        String::from_utf8_lossy(&s[k..])
                            .parse::<i32>()
                            .unwrap_or(256);
                    if !(0..=255).contains(&addr[3]) {
                        continue;
                    }
                    let [a, b, c, d] = addr;
                    res.push(format!("{a}.{b}.{c}.{d}"))
                }
            }
        }
    }
    res
}
