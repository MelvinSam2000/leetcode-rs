/*
    1108 - Defanging IP address
    Time: O(n)
    Space: O(n)
*/
pub fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}
