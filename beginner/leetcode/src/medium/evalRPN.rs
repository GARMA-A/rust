use std::collections::VecDeque;

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: VecDeque<i32> = VecDeque::new();

    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                // Pop right operand first, then left operand.
                let right = stack.pop_back().expect("Not enough operands");
                let left = stack.pop_back().expect("Not enough operands");
                let result = match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => unreachable!(),
                };
                // Push the result back onto the stack.
                stack.push_back(result);
            }
            num_str => {
                // Parse the number and push it onto the stack.
                let num = num_str.parse::<i32>().expect("Invalid number");
                stack.push_back(num);
            }
        }
    }

    // The final result should be the only element left in the stack.
    stack.pop_back().expect("Stack is empty")
}