impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len()/2);
        for bracket in s.chars() {
            if opening(bracket) {
                stack.push(bracket);
            } else {
                if !oposite(stack.pop().unwrap_or(' '),bracket) {
                    return false;
                }
            }
        }
        if stack.len() > 0 {
            false
        } else {
            true
        }
    }
}

fn oposite(c: char, c2: char) -> bool {
    match (c,c2) {
        ('(',')') => true,
        ('{','}') => true,
        ('[',']') => true,
        _ => false,
    }
}

fn opening(c: char) -> bool {
    match c {
        '(' => true,
        '{' => true,
        '[' => true,
        _ => false,
        
    }
}
