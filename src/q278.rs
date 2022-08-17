pub struct Api(i32);

impl Api {
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.0
    }

    /*
        278 - First Bad Version
        Time: O(logn)
        Space: O(1)
    */
    pub fn first_bad_version(&self, n: i32) -> i32 {
        self.helper(1, n as i64)
    }

    fn helper(&self, l: i64, r: i64) -> i32 {
        let m = ((l + r) / 2) as i32;
        let is_bad_m = self.isBadVersion(m);
        let is_bad_m_l = self.isBadVersion(m - 1);
        let is_bad_m_r = self.isBadVersion(m + 1);
        if is_bad_m != is_bad_m_l {
            return m;
        }
        if is_bad_m != is_bad_m_r {
            return m + 1;
        }
        if is_bad_m {
            self.helper(l, m as i64 - 1)
        } else {
            self.helper(m as i64 + 1, r)
        }
    }
}

#[test]
fn t1() {
    let tcases = [(3, 4), (50, 100), (2000, 2001), (1702766719, 2126753390)];
    for (bad, n) in tcases {
        assert_eq!(Api(bad).first_bad_version(n), bad);
    }
}
