extern crate typescript_ts as ts;

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::Js::new(&guard);
    let ts = js.ts();

//    let kind = &ts::SyntaxKind { id: ts::SyntaxKindEnum::MultiLineCommentTrivia }; // TODO
//    let kind = &ts::SyntaxKind { id: 3 };
    let node = ts.createNode(ts::SyntaxKindEnum::MultiLineCommentTrivia, None, None);
    println!("kind: {:?}", node.kind());
}