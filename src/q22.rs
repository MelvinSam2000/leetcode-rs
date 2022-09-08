/*
    22 - Generate Parentheses
    Time: O(2^n)
    Space: O(n)
*/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    helper(1, 0, n, "(".to_owned(), &mut res);
    res
}

fn helper(
    open: i32,
    close: i32,
    n: i32,
    output: String,
    res: &mut Vec<String>,
) {
    if open == close && open == n {
        res.push(output.clone());
    }
    if open != n {
        helper(
            open + 1,
            close,
            n,
            output.clone() + "(",
            res,
        );
    }
    if open > close {
        helper(open, close + 1, n, output + ")", res);
    }
}
