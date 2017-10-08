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

    let condition = ts.createBinary(guard,
        &*paramName.as_Expression(),
        // &*operator,
        &*Box::from(ts::SyntaxKind_LessThanEqualsToken_new()),
        &*Box::from(ts.createLiteral_number(guard, 1)));

    let ifBody = ts.createBlock(guard,
        &[&*Box::from(ts.createReturn(guard, &*Box::from(ts.createLiteral_number(guard, 1))))],
        /*multiline*/ true);
    let decrementedArg = ts.createBinary(guard, &*paramName.as_Expression(), &*Box::from(ts::SyntaxKind_MinusToken_new()), &*Box::from(ts.createLiteral_number(guard, 1)));
    // let recurse = ts.createBinary(guard,
    //     &*Box::from(paramName),
    //     ts::SyntaxKind::AsteriskToken,
    //     ts.createCall(guard, functionName, /*typeArgs*/ None, [decrementedArg]));
    // let statements = &[
    //     &*Box::from(ts.createIf(guard, condition, ifBody)),
    //     &*Box::from(ts.createReturn(guard,
    //         recurse
    //     )),
    // ];

    // ts.createFunctionDeclaration(guard,
    //     /*decorators*/ None,
    //     /*modifiers*/&[&*Box::from(ts.createToken(ts::SyntaxKind::ExportKeyword))],
    //     /*asteriskToken*/ None,
    //     functionName,
    //     /*typeParameters*/ None,
    //     [parameter],
    //     /*returnType*/ ts.createKeywordTypeNode(guard, ts::SyntaxKind::NumberKeyword),
    //     ts.createBlock(guard, statements, /*multiline*/ true),
    // );

    Box::from(decrementedArg)
}

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::read_js();
    ts::eval_js(&guard, &js);
    let ts = ts::ts(&guard);

    let resultFile = ts.createSourceFile(&guard, "someFileName.ts", "", &*Box::from(ts::ScriptTarget_Latest_new()), /*setParentNodes*/ false, &*Box::from(ts::ScriptKind_TS_new()));

    let printerOptions: Box<ts::PrinterOptions> = Box::new(ts::ObjectBox::new(&guard));
    printerOptions.set_newLine(&guard, Some(ts::NewLineKind::LineFeed));
    let printer = ts.createPrinter(&guard, &*printerOptions);
    
    let node = &*makeFactorialFunction(&guard, &*ts);
    let result = printer.printNode(&guard, &*Box::from(ts::EmitHint_Unspecified_new()), node, &*resultFile);

    println!("result: {}", result);
}