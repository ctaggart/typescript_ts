#![allow(non_snake_case)]
#![allow(dead_code)]
// port of:
// Creating and Printing a TypeScript AST
// https://github.com/Microsoft/TypeScript/wiki/Using-the-Compiler-API#creating-and-printing-a-typescript-ast

extern crate typescript_ts as ts;
extern crate chakracore;
use chakracore::context::ContextGuard;

fn makeFactorialFunction<'a>(ts: &'a ts::TsMod<'a>) -> ts::Node<'a> {

    let functionName = ts.createIdentifier("factorial");
    // let functionName2 = ts.createIdentifier("factorial"); // TODO problem with Box::from
    let paramName = ts.createIdentifier("n");
    let parameter = ts.createParameter(
        // /*decorators*/ None,
        // /*modifiers*/ None,
        // /*dotDotDotToken*/ None,
        &paramName);

    let condition = ts.createBinary(
        &paramName.as_Expression(),
        // &*operator,
        ts::SyntaxKind::LessThanEqualsToken,
        &ts.createLiteral_number(1).as_Expression());

    let ifBody = ts.createBlock(
        &[&ts.createReturn(&ts.createLiteral_number(1).as_Expression()).as_Statement()],
        /*multiline*/ true);
    let decrementedArg = ts.createBinary(&paramName.as_Expression(), ts::SyntaxKind::MinusToken, &ts.createLiteral_number(1).as_Expression());
    let recurse = ts.createBinary(
        &paramName.as_Expression(),
        ts::SyntaxKind::AsteriskToken,
        &ts.createCall(&functionName.as_Expression(), /*typeArgs*/ None, &[&decrementedArg.as_Expression()]).as_Expression());
    let statements = &[
        &ts.createIf(&condition.as_Expression(), &ifBody.as_Statement()).as_Statement(),
        &ts.createReturn(&recurse.as_Expression()).as_Statement(),
    ];

    let fd = ts.createFunctionDeclaration(
        /*decorators*/ None,
        /*modifiers*/Some(&[&ts.createToken(ts::SyntaxKind::ExportKeyword)]),
        /*asteriskToken*/ None,
        Some(&functionName),
        /*typeParameters*/ None,
        &[&parameter],
        /*returnType*/ Some(&ts.createKeywordTypeNode(ts::SyntaxKind::NumberKeyword).as_TypeNode()),
        Some(&ts.createBlock(statements, /*multiline*/ true)),
    );
    fd.as_Node()
}

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::Js::new(&guard);
    let ts = js.ts();

    let resultFile = ts.createSourceFile("someFileName.ts", "", ts::ScriptTarget::ESNext, /*setParentNodes*/ false, ts::ScriptKind::TS);

    let printerOptions = ts::PrinterOptions::new(&guard, chakracore::value::Object::new(&guard));
    printerOptions.set_newLine(Some(ts::NewLineKind::LineFeed));
    let printer = ts.createPrinter(&printerOptions);
    
    let node = makeFactorialFunction(&guard, &ts);
    let result = printer.printNode(ts::EmitHint::Unspecified, &node, &resultFile);

    println!("{}", result);
}