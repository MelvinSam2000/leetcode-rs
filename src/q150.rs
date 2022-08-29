/*
    150 - Evaluate reverse polish notation
    Time: O(n)
    Space: O(n)
*/
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        match token.as_str() {
            "+" => {
                let x1 = stack.pop().unwrap();
                let x2 = stack.pop().unwrap();
                stack.push(x1 + x2);
            }
            "-" => {
                let x1 = stack.pop().unwrap();
                let x2 = stack.pop().unwrap();
                stack.push(x2 - x1);
            }
            "*" => {
                let x1 = stack.pop().unwrap();
                let x2 = stack.pop().unwrap();
                stack.push(x1 * x2);
            }
            "/" => {
                let x1 = stack.pop().unwrap();
                let x2 = stack.pop().unwrap();
                stack.push(x2 / x1);
            }
            num => {
                stack.push(num.parse().unwrap());
            }
        }
    }
    stack.last().cloned().unwrap()
}
