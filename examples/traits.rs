trait Statement {}

trait IfStatement: Statement {}

struct IfStatementImpl;
impl Statement for IfStatementImpl {}
impl IfStatement for IfStatementImpl {}

fn print_statement(_statement: &Statement){
    println!("print_statement");
}

fn print_if_statement(_if_statement: &IfStatement){
    println!("print_if_statement");
}

fn main() {
    // How come this compiles?
    let ref a = IfStatementImpl {};
    print_statement(a);
    print_if_statement(a);

    // but this does not?
    // let ref b: IfStatement = IfStatementImpl {};
    let b: &IfStatement = &IfStatementImpl {};
    print_statement(b);
    print_if_statement(b);
}