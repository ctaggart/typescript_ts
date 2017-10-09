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
    let functionName2 = ts.createIdentifier(guard, "factorial"); // TODO problem with Box::from
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
    let recurse = ts.createBinary(guard,
        &*Box::from(paramName),
        &*Box::from(ts::SyntaxKind_AsteriskToken_new()),
        &*Box::from(ts.createCall(guard, &*Box::from(functionName), /*typeArgs*/ None, &[&*Box::from(decrementedArg)])));
    let statements = &[
        &*Box::from(ts.createIf(guard, &*Box::from(condition), &*Box::from(ifBody))),
        &*Box::from(ts.createReturn(guard,
            &*Box::from(recurse)
        )),
    ];

    let fnd = ts.createFunctionDeclaration(guard,
        /*decorators*/ None,
        /*modifiers*/Some(&[&*Box::from(ts.createToken(guard, &*Box::from(ts::SyntaxKind_ExportKeyword_new())))]),
        /*asteriskToken*/ None,
        Some(&*functionName2),
        /*typeParameters*/ None,
        &[&*parameter],
        /*returnType*/ Some(&*Box::from(ts.createKeywordTypeNode(guard, &*Box::from(ts::SyntaxKind_NumberKeyword_new())))),
        Some(&*ts.createBlock(guard, statements, /*multiline*/ true)),
    );

    Box::from(fnd)
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

    println!("{}", result);
}