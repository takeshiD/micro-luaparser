mod lexer;
use lexer::Lexer;
fn main() {
    let target = r#"
    local x = {}
    if x == nil then
        --this case is nil check
        print("x is nil")
    else
        --this case is not nil check
        print("x is not nil")
    end
    "#;
    let tokens = Lexer::new(target).tokenize();
    for tok in tokens {
        println!("{:?}", tok);
    }
}
