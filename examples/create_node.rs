extern crate typescript_ts as ts;

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::read_js();
    ts::eval_js(&guard, &js);
    let ts = ts::ts(&guard);

    let node = ts.createNode(&guard, ts::SyntaxKind::MultiLineCommentTrivia, None, None);
    println!("kind: {:?}", node.kind(&guard));
}