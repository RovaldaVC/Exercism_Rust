pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brackets = Vec::new();
    for w in string.chars(){
        if w == '(' || w == '[' || w =='{'{
            open_brackets.push(w);
        }
        if w == ')' || w == ']' || w =='}'{
            if let Some(open) = open_brackets.pop(){
                if !matches_pair(open, w){
                    return false;
                }
            }
            else{
                return false;
            }
        } 
    }
    open_brackets.is_empty()
}  

fn matches_pair(open:char, close:char) -> bool{
    (open == '(' && close == ')') ||
    (open == '[' && close == ']') ||
    (open == '{' && close == '}')
}

fn main() {}
