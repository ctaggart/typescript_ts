// port of:
// Creating and Printing a TypeScript AST
// https://github.com/Microsoft/TypeScript/wiki/Using-the-Compiler-API#creating-and-printing-a-typescript-ast

extern crate typescript_ts as ts;
extern crate chakracore;
use chakracore::context::ContextGuard;

fn makeFactorialFunction(guard: &ContextGuard, ts: &ts::TsMod){

    let functionName = ts.createIdentifier(guard, "factorial");
    let paramName = ts.createIdentifier(guard, "n");
    let parameter = ts.createParameter(guard,
        // /*decorators*/ None,
        // /*modifiers*/ None,
        // /*dotDotDotToken*/ None,
        &*paramName);

    let condition = ts.createBinary(guard,
        &*paramName.as_Expression(),
        &ts::SyntaxKind::LessThanEqualsToken,
        &ts.createLiteral_number(guard, 1));

    let ifBody = ts.createBlock(
        [ts.createReturn(guard, ts.createLiteral_number(guard, 1))],
        /*multiline*/ true);
    let decrementedArg = ts.createBinary(guard, &*paramName.as_Expression(), &ts::SyntaxKind::MinusToken, &ts.createLiteral(guard, 1));
    let recurse = ts.createBinary(guard,
        paramName,
        ts::SyntaxKind::AsteriskToken,
        ts.createCall(guard, functionName, /*typeArgs*/ None, [decrementedArg]));
    let statements = [
        ts.createIf(guard, condition, ifBody),
        ts.createReturn(guard,
            recurse
        )
    ];

    ts.createFunctionDeclaration(guard,
        /*decorators*/ None,
        /*modifiers*/[ts.createToken(ts::SyntaxKind::ExportKeyword)],
        /*asteriskToken*/ None,
        functionName,
        /*typeParameters*/ None,
        [parameter],
        /*returnType*/ ts.createKeywordTypeNode(guard, ts::SyntaxKind::NumberKeyword),
        ts.createBlock(guard, statements, /*multiline*/ true),
    );
}

fn main() {
    let (_runtime, context) = ts::new_context();
    let guard = ts::new_guard(&context);
    let js = ts::read_js();
    ts::eval_js(&guard, &js);
    let ts = ts::ts(&guard);

    let resultFile = ts.createSourceFile("someFileName.ts", "", ts::ScriptTarget::Latest, /*setParentNodes*/ false, ts::ScriptKind::TS);
    //     function createPrinter(printerOptions?: PrinterOptions, handlers?: PrintHandlers): Printer;
    //     interface PrinterOptions {
    //     removeComments?: boolean;
    //     newLine?: NewLineKind;
    // }
    // how to I create and set values?
    let printerOptions: Box<ts::PrinterOptions> = Box::new(ts::ObjectBox::new(&guard));
    printerOptions.set_newLine(&guard, Some(ts::NewLineKind::LineFeed));

    // let printer = ts.createPrinter({
    //     newLine: ts::NewLineKind::LineFeed
    // });
    // let result = printer.printNode(ts::EmitHint::Unspecified, makeFactorialFunction(), resultFile);

    // println!(result);
}