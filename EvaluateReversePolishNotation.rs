impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        eval(&mut tokens)
    }
}

fn eval(tokens: &mut Vec<String>) -> i32 {
    match tokens.pop().unwrap().as_str() {
        "*" => {
            let right = eval(tokens);
            let left = eval(tokens);
            left * right
        }
        "-" => {
            let right = eval(tokens);
            let left = eval(tokens);
            left - right
        }
        "+" => {
            let right = eval(tokens);
            let left = eval(tokens);
            left + right
        }
        "/" => {
            let right = eval(tokens);
            let left = eval(tokens);
            left / right
        }
        num => {
            num.parse::<i32>().unwrap()
        }
    }
}
