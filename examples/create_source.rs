#![allow(non_snake_case)]
#![allow(dead_code)]
// port of:
// Creating and Printing a TypeScript AST
// https://github.com/Microsoft/TypeScript/wiki/Using-the-Compiler-API#creating-and-printing-a-typescript-ast

extern crate typescript_ts as ts;
extern crate chakracore;
use chakracore::context::ContextGuard;

fn makeFactorialFunction(guard: &ContextGuard, ts: &ts::TsMod) -> Box<ts::Node> {

    let functionName = ts.createIdentifier(guard, "factorial");
    let paramName = ts.createIdentifier(guard, "n");
    let parameter = ts.createParameter(guard,
        // /*decorators*/ None,
        // /*modifiers*/ None,
        // /*dotDotDotToken*/ None,
        &*paramName);

    // // let operator: Box<ts::BinaryOperator> = Box::from(ts::new_SyntaxKind_LessThanEqualsToken());
    // let condition = ts.createBinary(guard,
    //     &*paramName.as_Expression(),
    //     // &*operator,
    //     &*Box::from(ts::new_SyntaxKind_LessThanEqualsToken()),
    //     &*Box::from(ts.createLiteral_number(guard, 1)));

    // let ifBody = ts.createBlock(
    //     [ts.createReturn(guard, ts.createLiteral_number(guard, 1))],
    //     /*multiline*/ true);
    // let decrementedArg = ts.createBinary(guard, &*paramName.as_Expression(), &ts::SyntaxKind::MinusToken, &ts.createLiteral(guard, 1));
    // let recurse = ts.createBinary(guard,
    //     paramName,
    //     ts::SyntaxKind::AsteriskToken,
    //     ts.createCall(guard, functionName, /*typeArgs*/ None, [decrementedArg]));
    // let statements = [
    //     ts.createIf(guard, condition, ifBody),
    //     ts.createReturn(guard,
    //         recurse
    //     )
    // ];

    // ts.createFunctionDeclaration(guard,
    //     /*decorators*/ None,
    //     /*modifiers*/[ts.createToken(ts::SyntaxKind::ExportKeyword)],
    //     /*asteriskToken*/ None,
    //     functionName,
    //     /*typeParameters*/ None,
    //     [parameter],
    //     /*returnType*/ ts.createKeywordTypeNode(guard, ts::SyntaxKind::NumberKeyword),
    //     ts.createBlock(guard, statements, /*multiline*/ true),
    // );

    Box::from(functionName)
}

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::read_js();
    ts::eval_js(&guard, &js);
    let ts = ts::ts(&guard);

    let resultFile = ts.createSourceFile(&guard, "someFileName.ts", "", &*Box::from(ts::new_ScriptTarget_Latest()), /*setParentNodes*/ false, &*Box::from(ts::new_ScriptKind_TS()));
    println!("fileName: {}", resultFile.fileName(&guard));

    let printerOptions: Box<ts::PrinterOptions> = Box::new(ts::ObjectBox::new(&guard));
    printerOptions.set_newLine(&guard, Some(ts::NewLineKind::LineFeed));

    // let printer = ts.createPrinter({
    //     newLine: ts::NewLineKind::LineFeed
    // });
    let printer = ts.createPrinter(&guard, &*printerOptions);
    
    let node = &*makeFactorialFunction(&guard, &*ts);
    println!("kind: {:?}", node.kind(&guard));
    let result = printer.printNode(&guard, &*Box::from(ts::new_EmitHint_Unspecified()), node, &*resultFile);

    println!("result: {}", result);
}