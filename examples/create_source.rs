// port of:
// Creating and Printing a TypeScript AST
// https://github.com/Microsoft/TypeScript/wiki/Using-the-Compiler-API#creating-and-printing-a-typescript-ast

extern crate typescript_ts as ts;
extern crate chakracore;
use chakracore::context::ContextGuard;

fn makeFactorialFunction(guard: &ContextGuard, ts: ts::TsMod){

    let functionName = ts.createIdentifier("factorial");
    let paramName = ts.createIdentifier("n");
    let parameter = ts.createParameter(
        /*decorators*/ None,
        /*modifiers*/ None,
        /*dotDotDotToken*/ None,
        paramName);

    let condition = ts.createBinary(
        paramName,
        ts::SyntaxKind.LessThanEqualsToken,
        ts.createLiteral(1));

    let ifBody = ts.createBlock(
        [ts.createReturn(ts.createLiteral(1))],
        /*multiline*/ true)
    let decrementedArg = ts.createBinary(paramName, ts.SyntaxKind.MinusToken, ts.createLiteral(1))
    let recurse = ts.createBinary(
        paramName,
        ts.SyntaxKind.AsteriskToken,
        ts.createCall(functionName, /*typeArgs*/ None, [decrementedArg]));
    let statements = [
        ts.createIf(condition, ifBody),
        ts.createReturn(
            recurse
        ),
    ];

    ts.createFunctionDeclaration(
        /*decorators*/ None,
        /*modifiers*/[ts.createToken(ts.SyntaxKind.ExportKeyword)],
        /*asteriskToken*/ None,
        functionName,
        /*typeParameters*/ None,
        [parameter],
        /*returnType*/ ts.createKeywordTypeNode(ts.SyntaxKind.NumberKeyword),
        ts.createBlock(statements, /*multiline*/ true),
    )
}

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::read_js();
    ts::eval_js(&guard, &js);
    let ts = ts::ts(&guard);

    let resultFile = ts.createSourceFile("someFileName.ts", "", ts::ScriptTarget::Latest, /*setParentNodes*/ false, ts::ScriptKind::TS);
    let printer = ts.createPrinter({
        newLine: ts::NewLineKind::LineFeed,
    });
    let result = printer.printNode(ts::EmitHint::Unspecified, makeFactorialFunction(), resultFile);

    println!(result);
}