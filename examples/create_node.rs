extern crate typescript_ts as ts;

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::Js::new(&guard);
    let ts = js.ts();

    let node = ts.createNode(ts::SyntaxKind::MultiLineCommentTrivia, None, None);
    println!("kind: {:?}", node.kind());
}