#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
extern crate chakracore;
use std::io::prelude::*;
use std::fs::File;

pub fn read_js() -> String {
    let file = "./node_modules/typescript/lib/typescript.js";
    let mut f = File::open(file).expect("unable to open the file");
    let mut js = String::new();
    f.read_to_string(&mut js).expect("unable to read the file");
    js
}

pub struct Js;

impl Js {
    pub fn version(guard: &chakracore::context::ContextGuard) -> String {
        guard.global().get(guard, &chakracore::Property::new(guard, "ts")).to_string(guard)
    }
}

pub trait GetObject {
    fn object(&self) -> &chakracore::value::Object;
}

pub struct ObjectBox {
    object: chakracore::value::Object,
}

impl ObjectBox {
    pub fn new(guard: &chakracore::context::ContextGuard) -> Self {
        ObjectBox { object: chakracore::value::Object::new(guard) }
    }
}

impl GetObject for ObjectBox {
    fn object(&self) -> &chakracore::value::Object {
        &self.object
    }
}

pub trait GetNumber {
    fn number(&self) -> &chakracore::value::Number;
}

pub struct NumberBox {
    number: chakracore::value::Number,
}

impl NumberBox {
    pub fn new(guard: &chakracore::context::ContextGuard, number: i32) -> NumberBox {
        NumberBox { number: chakracore::value::Number::new(guard, number) }
    }
}

impl GetNumber for NumberBox {
    fn number(&self) -> &chakracore::value::Number {
        &self.number
    }
}

pub trait GetId {
    fn id(&self) -> i32;
}

pub struct IdBox {
    id: i32,
}

impl GetId for IdBox {
    fn id(&self) -> i32 {
        self.id
    }
}

pub struct Enum;

pub fn new_context() -> (chakracore::Runtime, chakracore::Context) {
    let runtime = chakracore::Runtime::new().unwrap();
    let context = chakracore::Context::new(&runtime).unwrap();
    (runtime, context)
}

pub fn new_guard(context: &chakracore::Context) -> chakracore::context::ContextGuard {
    context.make_current().unwrap()
}

pub fn eval_js(guard: &chakracore::context::ContextGuard, js: &str) {
    chakracore::script::eval(guard, js).expect("invalid JavaScript code");
}

/// get the `ts` variable that exposes the TypeScript module
pub fn ts(guard: &chakracore::context::ContextGuard) -> Box<TsMod> {
    let ts = guard.global().get(guard, &chakracore::Property::new(guard, "ts")).into_object().unwrap();
    Box::new(ObjectBox { object: ts })
}

/// TypeScript module exposed by the `ts` variable
pub trait TsMod : GetObject {
    fn version(&self, guard: &chakracore::context::ContextGuard) -> String {
        self.object().get(guard, &chakracore::Property::new(guard, "version")).to_string(guard)
    }

    // createNode(kind: SyntaxKind, pos?: number, end?: number): Node;
    fn createNode(&self, guard: &chakracore::context::ContextGuard, kind: &SyntaxKind, pos: Option<i32>, end: Option<i32> ) -> Box<Node> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createNode")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::Number::new(guard, kind.id()).into(),
            &chakracore::value::Number::new(guard, pos.unwrap_or(-1)).into(),
            &chakracore::value::Number::new(guard, end.unwrap_or(-1)).into(),
        ]);
        let node = rv.unwrap().into_object().unwrap();
        Box::new(ObjectBox { object: node })
    }

    // function createParameter(decorators: ReadonlyArray<Decorator> | undefined, modifiers: ReadonlyArray<Modifier> | undefined, dotDotDotToken: DotDotDotToken | undefined, name: string | BindingName, questionToken?: QuestionToken, type?: TypeNode, initializer?: Expression): ParameterDeclaration;
    fn createParameter(&self, guard: &chakracore::context::ContextGuard, name: &Identifier) -> Box<ParameterDeclaration> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createParameter")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::undefined(guard),
            &chakracore::value::undefined(guard),
            &chakracore::value::undefined(guard),
            name.object(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createIdentifier(text: string): Identifier;
    fn createIdentifier(&self, guard: &chakracore::context::ContextGuard, text: &str) -> Box<Identifier> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createIdentifier")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::String::new(guard, text).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createLiteral(value: number): NumericLiteral;
    fn createLiteral_number(&self, guard: &chakracore::context::ContextGuard, value: i32) -> Box<NumericLiteral> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createLiteral")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::Number::new(guard, value).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createBinary(left: Expression, operator: BinaryOperator | BinaryOperatorToken, right: Expression): BinaryExpression;
    fn createBinary(&self, guard: &chakracore::context::ContextGuard, left: &Expression, operator: &BinaryOperator, right: &Expression) -> Box<BinaryExpression> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createBinary")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            left.object(),
            &chakracore::value::Number::new(guard, operator.id()).into(),
            right.object(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createBlock(statements: ReadonlyArray<Statement>, multiLine?: boolean): Block;
    fn createBlock(&self, guard: &chakracore::context::ContextGuard, statements: &[&Statement], multiLine: bool ) -> Box<Block> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createBlock")).into_function().unwrap();

        let statements_length = statements.len() as u32;
        let statements_array = chakracore::value::Array::new(guard, statements_length);
        for i in 0..statements_length {
            statements_array.set_index(guard, i, statements[i as usize].object());
        }

        let rv = function.call_with_this(guard, this, &[
            &statements_array,
            &chakracore::value::Boolean::new(guard, multiLine).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createSourceFile(fileName: string, sourceText: string, languageVersion: ScriptTarget, setParentNodes?: boolean, scriptKind?: ScriptKind): SourceFile;
    // ts.createSourceFile("someFileName.ts", "", ts::ScriptTarget::Latest, /*setParentNodes*/ false, ts::ScriptKind::TS);
    fn createSourceFile(&self, guard: &chakracore::context::ContextGuard, fileName: &str, sourceText: &str, languageVersion: &ScriptTarget, setParentNodes: bool, scriptKind: &ScriptKind) -> Box<SourceFile> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createSourceFile")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::String::new(guard, fileName).into(),
            &chakracore::value::String::new(guard, sourceText).into(),
            &chakracore::value::Number::new(guard, languageVersion.id()).into(),
            &chakracore::value::Boolean::new(guard, setParentNodes).into(),
            &chakracore::value::Number::new(guard, scriptKind.id()).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createPrinter(printerOptions?: PrinterOptions, handlers?: PrintHandlers): Printer;
    fn createPrinter(&self, guard: &chakracore::context::ContextGuard, printerOptions: &PrinterOptions) -> Box<Printer> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createPrinter")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            printerOptions.object()
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createReturn(expression?: Expression): ReturnStatement;
    fn createReturn(&self, guard: &chakracore::context::ContextGuard, expression: &Expression) -> Box<ReturnStatement> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createReturn")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            expression.object(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createCall(expression: Expression, typeArguments: ReadonlyArray<TypeNode> | undefined, argumentsArray: ReadonlyArray<Expression>): CallExpression;
    fn createCall(&self, guard: &chakracore::context::ContextGuard, expression: &Expression, typeArguments: Option<&[&TypeNode]>, argumentsArray: &[&Expression]) -> Box<CallExpression> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createCall")).into_function().unwrap();
        
        let argumentsArray_length = argumentsArray.len() as u32;
        let argumentsArray_array = chakracore::value::Array::new(guard, argumentsArray_length);
        for i in 0..argumentsArray_length {
            argumentsArray_array.set_index(guard, i, argumentsArray[i as usize].object());
        }
        
        let rv = function.call_with_this(guard, this, &[
            expression.object(),
            &chakracore::value::undefined(guard), // TODO typeArguments
            &argumentsArray_array,
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createIf(expression: Expression, thenStatement: Statement, elseStatement?: Statement): IfStatement;
    fn createIf(&self, guard: &chakracore::context::ContextGuard, expression: &Expression, thenStatement: &Statement) -> Box<IfStatement> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createIf")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            expression.object(),
            thenStatement.object(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createKeywordTypeNode(kind: KeywordTypeNode["kind"]): KeywordTypeNode;
    fn createKeywordTypeNode(&self, guard: &chakracore::context::ContextGuard, kind: &SyntaxKind) -> Box<KeywordTypeNode> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createKeywordTypeNode")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::Number::new(guard, kind.id()).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createToken<TKind extends SyntaxKind>(token: TKind): Token<TKind>;
    fn createToken(&self, guard: &chakracore::context::ContextGuard, token: &SyntaxKind) -> Box<Token> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createToken")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::Number::new(guard, token.id()).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createFunctionDeclaration(decorators: ReadonlyArray<Decorator> | undefined, modifiers: ReadonlyArray<Modifier> | undefined, asteriskToken: AsteriskToken | undefined, name: string | Identifier | undefined, typeParameters: ReadonlyArray<TypeParameterDeclaration> | undefined, parameters: ReadonlyArray<ParameterDeclaration>, type: TypeNode | undefined, body: Block | undefined): FunctionDeclaration;
    fn createFunctionDeclaration(&self, guard: &chakracore::context::ContextGuard, decorators: Option<&[&Decorator]>, modifiers: Option<&[&Token]>, asteriskToken: Option<&AsteriskToken>, name: Option<&Identifier>, typeParameters: Option<&[&TypeParameterDeclaration]>, parameters: &[&ParameterDeclaration], type_: Option<&TypeNode>, body: Option<&Block>) -> Box<FunctionDeclaration> {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "createFunctionDeclaration")).into_function().unwrap();
        
        let modifiers = modifiers.unwrap();
        let modifiers_length = modifiers.len() as u32;
        let modifiers_array = chakracore::value::Array::new(guard, modifiers_length);
        for i in 0..modifiers_length {
            modifiers_array.set_index(guard, i, modifiers[i as usize].object());
        }

        let parameters_length = parameters.len() as u32;
        let parameters_array = chakracore::value::Array::new(guard, parameters_length);
        for i in 0..parameters_length {
            parameters_array.set_index(guard, i, parameters[i as usize].object());
        }
        
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::undefined(guard), // TODO decorators
            &modifiers_array,
            &chakracore::value::undefined(guard), // TODO asteriskToken
            name.unwrap().object(),
            &chakracore::value::undefined(guard), // TODO typeParameters
            &parameters_array,
            type_.unwrap().object(),
            body.unwrap().object(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

}
impl TsMod for ObjectBox {}

pub trait Node: GetObject + TextRange {
    fn kind(&self, guard: &chakracore::context::ContextGuard) -> i32 {
        let kind = self.object().get(&guard, &chakracore::Property::new(&guard, "kind"));
        kind.into_number().unwrap().value()
    }
}
impl Node for ObjectBox {}

pub trait TextRange {}
impl TextRange for ObjectBox {}

pub trait Declaration: Node {}

impl Declaration for ObjectBox {}

pub trait SourceFile: Declaration {
    fn fileName(&self, guard: &chakracore::context::ContextGuard) -> String {
        let this = self.object();
        this.get(guard, &chakracore::Property::new(guard, "fileName")).into_string().unwrap().value()
    }
}
impl SourceFile for ObjectBox {}

// interface Expression extends Node {
//     _expressionBrand: any;
// }
pub trait Expression: Node {}
impl Expression for ObjectBox {}

// https://stackoverflow.com/a/28664881/23059
pub trait AsExpression {
    fn as_Expression(&self) -> &Expression;
}
impl<T: Expression> AsExpression for T {
    fn as_Expression(&self) -> &Expression { self }
}

pub trait UnaryExpression: Expression {}
impl UnaryExpression for ObjectBox {}

pub trait UpdateExpression: UnaryExpression {}
impl UpdateExpression for ObjectBox {}

pub trait LeftHandSideExpression: UpdateExpression {}
impl LeftHandSideExpression for ObjectBox {}

pub trait MemberExpression: LeftHandSideExpression {}
impl MemberExpression for ObjectBox {}

pub trait PrimaryExpression: MemberExpression {}
impl PrimaryExpression for ObjectBox {}

pub trait Identifier: GetObject + PrimaryExpression + AsExpression {}
impl Identifier for ObjectBox {}

pub trait NumericLiteral: GetObject {}
impl NumericLiteral for ObjectBox {}

impl From<Box<NumericLiteral>> for Box<Expression> {
    fn from(v: Box<NumericLiteral>) -> Box<Expression> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

pub trait ParameterDeclaration: GetObject {}
impl ParameterDeclaration for ObjectBox {}

pub trait PrinterOptions : GetObject  {
    fn set_newLine(&self, guard: &chakracore::context::ContextGuard, value: Option<NewLineKind>) {
        let property = &chakracore::Property::new(guard, "newLine");
        match value {
            None => {
                let jsv = &chakracore::value::undefined(guard);
                self.object().set(guard, property, jsv);
            },
            Some(v) => {
                let jsv = &chakracore::value::Number::new(guard, i32::from(v));
                self.object().set(guard, property, jsv);
            },
        }
    }
}
impl PrinterOptions for ObjectBox {}

pub trait Printer : GetObject  {

    // printNode(hint: EmitHint, node: Node, sourceFile: SourceFile): string;
    fn printNode(&self, guard: &chakracore::context::ContextGuard, hint: &EmitHint, node: &Node, sourceFile: &SourceFile) -> String {
        let this = self.object();
        let function = this.get(guard, &chakracore::Property::new(guard, "printNode")).into_function().unwrap();
        let rv = function.call_with_this(guard, this, &[
            &chakracore::value::Number::new(guard, hint.id()).into(),
            node.object(),
            sourceFile.object(),
        ]);
        rv.unwrap().into_string().unwrap().value()
    }
}
impl Printer for ObjectBox {}

pub trait BinaryExpression: Expression + Declaration {}
impl BinaryExpression for ObjectBox {}

pub trait SyntaxKind: GetId {}
impl SyntaxKind for IdBox {}

pub trait SyntaxKind_Unknown: GetId { 
    fn id(&self) -> i32 { 
        SyntaxKindEnum::Unknown as i32
    }
}
impl From<Box<SyntaxKind_Unknown>> for Box<SyntaxKind> {
    fn from(v: Box<SyntaxKind_Unknown>) -> Box<SyntaxKind> {
        Box::new(IdBox { id: SyntaxKind_Unknown::id(&*v) })
    }
}

pub trait BinaryOperator: GetId {}
impl BinaryOperator for IdBox {}

pub trait SyntaxKind_LessThanEqualsToken {}
impl GetId for SyntaxKind_LessThanEqualsToken {
    fn id(&self) -> i32 { 
        SyntaxKindEnum::LessThanEqualsToken as i32
    }
}
impl SyntaxKind_LessThanEqualsToken for Enum {}
pub fn SyntaxKind_LessThanEqualsToken_new() -> Box<SyntaxKind_LessThanEqualsToken> {
    Box::new(Enum {})
}

pub trait SyntaxKind_MinusToken {}
impl GetId for SyntaxKind_MinusToken {
    fn id(&self) -> i32 { 
        SyntaxKindEnum::MinusToken as i32
    }
}
impl SyntaxKind_MinusToken for Enum {}
pub fn SyntaxKind_MinusToken_new() -> Box<SyntaxKind_MinusToken> {
    Box::new(Enum {})
}
impl From<Box<SyntaxKind_MinusToken>> for Box<SyntaxKind> {
    fn from(v: Box<SyntaxKind_MinusToken>) -> Box<SyntaxKind> {
        Box::new(IdBox { id: SyntaxKind_MinusToken::id(&*v) })
    }
}
impl From<Box<SyntaxKind_MinusToken>> for Box<BinaryOperator> {
    fn from(v: Box<SyntaxKind_MinusToken>) -> Box<BinaryOperator> {
        Box::new(IdBox { id: SyntaxKind_MinusToken::id(&*v) })
    }
}

pub trait SyntaxKind_AsteriskToken {}
impl GetId for SyntaxKind_AsteriskToken {
    fn id(&self) -> i32 { 
        SyntaxKindEnum::AsteriskToken as i32
    }
}
impl SyntaxKind_AsteriskToken for Enum {}
pub fn SyntaxKind_AsteriskToken_new() -> Box<SyntaxKind_AsteriskToken> {
    Box::new(Enum {})
}
impl From<Box<SyntaxKind_AsteriskToken>> for Box<SyntaxKind> {
    fn from(v: Box<SyntaxKind_AsteriskToken>) -> Box<SyntaxKind> {
        Box::new(IdBox { id: SyntaxKind_AsteriskToken::id(&*v) })
    }
}
impl From<Box<SyntaxKind_AsteriskToken>> for Box<BinaryOperator> {
    fn from(v: Box<SyntaxKind_AsteriskToken>) -> Box<BinaryOperator> {
        Box::new(IdBox { id: SyntaxKind_AsteriskToken::id(&*v) })
    }
}

pub trait SyntaxKind_ExportKeyword {}
impl GetId for SyntaxKind_ExportKeyword {
    fn id(&self) -> i32 { 
        SyntaxKindEnum::ExportKeyword as i32
    }
}
impl SyntaxKind_ExportKeyword for Enum {}
pub fn SyntaxKind_ExportKeyword_new() -> Box<SyntaxKind_ExportKeyword> {
    Box::new(Enum {})
}
impl From<Box<SyntaxKind_ExportKeyword>> for Box<SyntaxKind> {
    fn from(v: Box<SyntaxKind_ExportKeyword>) -> Box<SyntaxKind> {
        Box::new(IdBox { id: SyntaxKind_ExportKeyword::id(&*v) })
    }
}
impl From<Box<SyntaxKind_ExportKeyword>> for Box<BinaryOperator> {
    fn from(v: Box<SyntaxKind_ExportKeyword>) -> Box<BinaryOperator> {
        Box::new(IdBox { id: SyntaxKind_ExportKeyword::id(&*v) })
    }
}

pub trait SyntaxKind_NumberKeyword {}
impl GetId for SyntaxKind_NumberKeyword {
    fn id(&self) -> i32 { 
        SyntaxKindEnum::NumberKeyword as i32
    }
}
impl SyntaxKind_NumberKeyword for Enum {}
pub fn SyntaxKind_NumberKeyword_new() -> Box<SyntaxKind_NumberKeyword> {
    Box::new(Enum {})
}
impl From<Box<SyntaxKind_NumberKeyword>> for Box<SyntaxKind> {
    fn from(v: Box<SyntaxKind_NumberKeyword>) -> Box<SyntaxKind> {
        Box::new(IdBox { id: SyntaxKind_NumberKeyword::id(&*v) })
    }
}
impl From<Box<SyntaxKind_NumberKeyword>> for Box<BinaryOperator> {
    fn from(v: Box<SyntaxKind_NumberKeyword>) -> Box<BinaryOperator> {
        Box::new(IdBox { id: SyntaxKind_NumberKeyword::id(&*v) })
    }
}

impl From<Box<SyntaxKind_LessThanEqualsToken>> for Box<SyntaxKind> {
    fn from(v: Box<SyntaxKind_LessThanEqualsToken>) -> Box<SyntaxKind> {
        Box::new(IdBox { id: SyntaxKind_LessThanEqualsToken::id(&*v) })
    }
}
impl From<Box<SyntaxKind_LessThanEqualsToken>> for Box<BinaryOperator> {
    fn from(v: Box<SyntaxKind_LessThanEqualsToken>) -> Box<BinaryOperator> {
        Box::new(IdBox { id: SyntaxKind_LessThanEqualsToken::id(&*v) })
    }
}

pub enum SyntaxKindEnum {
    Unknown = 0,
    EndOfFileToken = 1,
    SingleLineCommentTrivia = 2,
    MultiLineCommentTrivia = 3,
    NewLineTrivia = 4,
    WhitespaceTrivia = 5,
    ShebangTrivia = 6,
    ConflictMarkerTrivia = 7,
    NumericLiteral = 8,
    StringLiteral = 9,
    JsxText = 10,
    JsxTextAllWhiteSpaces = 11,
    RegularExpressionLiteral = 12,
    NoSubstitutionTemplateLiteral = 13,
    TemplateHead = 14,
    TemplateMiddle = 15,
    TemplateTail = 16,
    OpenBraceToken = 17,
    CloseBraceToken = 18,
    OpenParenToken = 19,
    CloseParenToken = 20,
    OpenBracketToken = 21,
    CloseBracketToken = 22,
    DotToken = 23,
    DotDotDotToken = 24,
    SemicolonToken = 25,
    CommaToken = 26,
    LessThanToken = 27,
    LessThanSlashToken = 28,
    GreaterThanToken = 29,
    LessThanEqualsToken = 30,
    GreaterThanEqualsToken = 31,
    EqualsEqualsToken = 32,
    ExclamationEqualsToken = 33,
    EqualsEqualsEqualsToken = 34,
    ExclamationEqualsEqualsToken = 35,
    EqualsGreaterThanToken = 36,
    PlusToken = 37,
    MinusToken = 38,
    AsteriskToken = 39,
    AsteriskAsteriskToken = 40,
    SlashToken = 41,
    PercentToken = 42,
    PlusPlusToken = 43,
    MinusMinusToken = 44,
    LessThanLessThanToken = 45,
    GreaterThanGreaterThanToken = 46,
    GreaterThanGreaterThanGreaterThanToken = 47,
    AmpersandToken = 48,
    BarToken = 49,
    CaretToken = 50,
    ExclamationToken = 51,
    TildeToken = 52,
    AmpersandAmpersandToken = 53,
    BarBarToken = 54,
    QuestionToken = 55,
    ColonToken = 56,
    AtToken = 57,
    EqualsToken = 58,
    PlusEqualsToken = 59,
    MinusEqualsToken = 60,
    AsteriskEqualsToken = 61,
    AsteriskAsteriskEqualsToken = 62,
    SlashEqualsToken = 63,
    PercentEqualsToken = 64,
    LessThanLessThanEqualsToken = 65,
    GreaterThanGreaterThanEqualsToken = 66,
    GreaterThanGreaterThanGreaterThanEqualsToken = 67,
    AmpersandEqualsToken = 68,
    BarEqualsToken = 69,
    CaretEqualsToken = 70,
    Identifier = 71,
    BreakKeyword = 72,
    CaseKeyword = 73,
    CatchKeyword = 74,
    ClassKeyword = 75,
    ConstKeyword = 76,
    ContinueKeyword = 77,
    DebuggerKeyword = 78,
    DefaultKeyword = 79,
    DeleteKeyword = 80,
    DoKeyword = 81,
    ElseKeyword = 82,
    traitKeyword = 83,
    ExportKeyword = 84,
    ExtendsKeyword = 85,
    FalseKeyword = 86,
    FinallyKeyword = 87,
    ForKeyword = 88,
    FunctionKeyword = 89,
    IfKeyword = 90,
    ImportKeyword = 91,
    InKeyword = 92,
    InstanceOfKeyword = 93,
    NewKeyword = 94,
    NullKeyword = 95,
    ReturnKeyword = 96,
    SuperKeyword = 97,
    SwitchKeyword = 98,
    ThisKeyword = 99,
    ThrowKeyword = 100,
    TrueKeyword = 101,
    TryKeyword = 102,
    TypeOfKeyword = 103,
    VarKeyword = 104,
    VoidKeyword = 105,
    WhileKeyword = 106,
    WithKeyword = 107,
    ImplementsKeyword = 108,
    InterfaceKeyword = 109,
    LetKeyword = 110,
    PackageKeyword = 111,
    PrivateKeyword = 112,
    ProtectedKeyword = 113,
    PublicKeyword = 114,
    StaticKeyword = 115,
    YieldKeyword = 116,
    AbstractKeyword = 117,
    AsKeyword = 118,
    AnyKeyword = 119,
    AsyncKeyword = 120,
    AwaitKeyword = 121,
    BooleanKeyword = 122,
    ConstructorKeyword = 123,
    DeclareKeyword = 124,
    GetKeyword = 125,
    IsKeyword = 126,
    KeyOfKeyword = 127,
    ModuleKeyword = 128,
    NamespaceKeyword = 129,
    NeverKeyword = 130,
    ReadonlyKeyword = 131,
    RequireKeyword = 132,
    NumberKeyword = 133,
    ObjectKeyword = 134,
    SetKeyword = 135,
    StringKeyword = 136,
    SymbolKeyword = 137,
    TypeKeyword = 138,
    UndefinedKeyword = 139,
    FromKeyword = 140,
    GlobalKeyword = 141,
    OfKeyword = 142,
    QualifiedName = 143,
    ComputedPropertyName = 144,
    TypeParameter = 145,
    Parameter = 146,
    Decorator = 147,
    PropertySignature = 148,
    PropertyDeclaration = 149,
    MethodSignature = 150,
    MethodDeclaration = 151,
    Constructor = 152,
    GetAccessor = 153,
    SetAccessor = 154,
    CallSignature = 155,
    ConstructSignature = 156,
    IndexSignature = 157,
    TypePredicate = 158,
    TypeReference = 159,
    FunctionType = 160,
    ConstructorType = 161,
    TypeQuery = 162,
    TypeLiteral = 163,
    ArrayType = 164,
    TupleType = 165,
    UnionType = 166,
    IntersectionType = 167,
    ParenthesizedType = 168,
    ThisType = 169,
    TypeOperator = 170,
    IndexedAccessType = 171,
    MappedType = 172,
    LiteralType = 173,
    ObjectBindingPattern = 174,
    ArrayBindingPattern = 175,
    BindingElement = 176,
    ArrayLiteralExpression = 177,
    ObjectLiteralExpression = 178,
    PropertyAccessExpression = 179,
    ElementAccessExpression = 180,
    CallExpression = 181,
    NewExpression = 182,
    TaggedTemplateExpression = 183,
    TypeAssertionExpression = 184,
    ParenthesizedExpression = 185,
    FunctionExpression = 186,
    ArrowFunction = 187,
    DeleteExpression = 188,
    TypeOfExpression = 189,
    VoidExpression = 190,
    AwaitExpression = 191,
    PrefixUnaryExpression = 192,
    PostfixUnaryExpression = 193,
    BinaryExpression = 194,
    ConditionalExpression = 195,
    TemplateExpression = 196,
    YieldExpression = 197,
    SpreadElement = 198,
    ClassExpression = 199,
    OmittedExpression = 200,
    ExpressionWithTypeArguments = 201,
    AsExpression = 202,
    NonNullExpression = 203,
    MetaProperty = 204,
    TemplateSpan = 205,
    SemicolonClassElement = 206,
    Block = 207,
    VariableStatement = 208,
    EmptyStatement = 209,
    ExpressionStatement = 210,
    IfStatement = 211,
    DoStatement = 212,
    WhileStatement = 213,
    ForStatement = 214,
    ForInStatement = 215,
    ForOfStatement = 216,
    ContinueStatement = 217,
    BreakStatement = 218,
    ReturnStatement = 219,
    WithStatement = 220,
    SwitchStatement = 221,
    LabeledStatement = 222,
    ThrowStatement = 223,
    TryStatement = 224,
    DebuggerStatement = 225,
    VariableDeclaration = 226,
    VariableDeclarationList = 227,
    FunctionDeclaration = 228,
    ClassDeclaration = 229,
    InterfaceDeclaration = 230,
    TypeAliasDeclaration = 231,
    traitDeclaration = 232,
    ModuleDeclaration = 233,
    ModuleBlock = 234,
    CaseBlock = 235,
    NamespaceExportDeclaration = 236,
    ImportEqualsDeclaration = 237,
    ImportDeclaration = 238,
    ImportClause = 239,
    NamespaceImport = 240,
    NamedImports = 241,
    ImportSpecifier = 242,
    ExportAssignment = 243,
    ExportDeclaration = 244,
    NamedExports = 245,
    ExportSpecifier = 246,
    MissingDeclaration = 247,
    ExternalModuleReference = 248,
    JsxElement = 249,
    JsxSelfClosingElement = 250,
    JsxOpeningElement = 251,
    JsxClosingElement = 252,
    JsxAttribute = 253,
    JsxAttributes = 254,
    JsxSpreadAttribute = 255,
    JsxExpression = 256,
    CaseClause = 257,
    DefaultClause = 258,
    HeritageClause = 259,
    CatchClause = 260,
    PropertyAssignment = 261,
    ShorthandPropertyAssignment = 262,
    SpreadAssignment = 263,
    traitMember = 264,
    SourceFile = 265,
    Bundle = 266,
    JSDocTypeExpression = 267,
    JSDocAllType = 268,
    JSDocUnknownType = 269,
    JSDocArrayType = 270,
    JSDocUnionType = 271,
    JSDocTupleType = 272,
    JSDocNullableType = 273,
    JSDocNonNullableType = 274,
    JSDocRecordType = 275,
    JSDocRecordMember = 276,
    JSDocTypeReference = 277,
    JSDocOptionalType = 278,
    JSDocFunctionType = 279,
    JSDocVariadicType = 280,
    JSDocConstructorType = 281,
    JSDocThisType = 282,
    JSDocComment = 283,
    JSDocTag = 284,
    JSDocAugmentsTag = 285,
    JSDocClassTag = 286,
    JSDocParameterTag = 287,
    JSDocReturnTag = 288,
    JSDocTypeTag = 289,
    JSDocTemplateTag = 290,
    JSDocTypedefTag = 291,
    JSDocPropertyTag = 292,
    JSDocTypeLiteral = 293,
    JSDocLiteralType = 294,
    SyntaxList = 295,
    NotEmittedStatement = 296,
    PartiallyEmittedExpression = 297,
    CommaListExpression = 298,
    MergeDeclarationMarker = 299,
    EndOfDeclarationMarker = 300,
}

pub trait SyntaxKindConst {
    const Count: i32 = 301;
    const FirstAssignment: i32 = 58;
    const LastAssignment: i32 = 70;
    const FirstCompoundAssignment: i32 = 59;
    const LastCompoundAssignment: i32 = 70;
    const FirstReservedWord: i32 = 72;
    const LastReservedWord: i32 = 107;
    const FirstKeyword: i32 = 72;
    const LastKeyword: i32 = 142;
    const FirstFutureReservedWord: i32 = 108;
    const LastFutureReservedWord: i32 = 116;
    const FirstTypeNode: i32 = 158;
    const LastTypeNode: i32 = 173;
    const FirstPunctuation: i32 = 17;
    const LastPunctuation: i32 = 70;
    const FirstToken: i32 = 0;
    const LastToken: i32 = 142;
    const FirstTriviaToken: i32 = 2;
    const LastTriviaToken: i32 = 7;
    const FirstLiteralToken: i32 = 8;
    const LastLiteralToken: i32 = 13;
    const FirstTemplateToken: i32 = 13;
    const LastTemplateToken: i32 = 16;
    const FirstBinaryOperator: i32 = 27;
    const LastBinaryOperator: i32 = 70;
    const FirstNode: i32 = 143;
    const FirstJSDocNode: i32 = 267;
    const LastJSDocNode: i32 = 294;
    const FirstJSDocTagNode: i32 = 284;
    const LastJSDocTagNode: i32 = 294;
}


pub struct NodeFlags(i32);
impl NodeFlags {
    pub const None: i32 = 0;
    pub const Let: i32 = 1;
    pub const Const: i32 = 2;
    pub const NestedNamespace: i32 = 4;
    pub const Synthesized: i32 = 8;
    pub const Namespace: i32 = 16;
    pub const ExportContext: i32 = 32;
    pub const ContainsThis: i32 = 64;
    pub const HasImplicitReturn: i32 = 128;
    pub const HasExplicitReturn: i32 = 256;
    pub const GlobalAugmentation: i32 = 512;
    pub const HasAsyncFunctions: i32 = 1024;
    pub const DisallowInContext: i32 = 2048;
    pub const YieldContext: i32 = 4096;
    pub const DecoratorContext: i32 = 8192;
    pub const AwaitContext: i32 = 16384;
    pub const ThisNodeHasError: i32 = 32768;
    pub const JavaScriptFile: i32 = 65536;
    pub const ThisNodeOrAnySubNodesHasError: i32 = 131072;
    pub const HasAggregatedChildData: i32 = 262144;
    pub const BlockScoped: i32 = 3;
    pub const ReachabilityCheckFlags: i32 = 384;
    pub const ReachabilityAndEmitFlags: i32 = 1408;
    pub const ContextFlags: i32 = 96256;
    pub const TypeExcludesFlags: i32 = 20480;
}
pub struct ModifierFlags(i32);
impl ModifierFlags {
    pub const None: i32 = 0;
    pub const Export: i32 = 1;
    pub const Ambient: i32 = 2;
    pub const Public: i32 = 4;
    pub const Private: i32 = 8;
    pub const Protected: i32 = 16;
    pub const Static: i32 = 32;
    pub const Readonly: i32 = 64;
    pub const Abstract: i32 = 128;
    pub const Async: i32 = 256;
    pub const Default: i32 = 512;
    pub const Const: i32 = 2048;
    pub const HasComputedFlags: i32 = 536870912;
    pub const AccessibilityModifier: i32 = 28;
    pub const ParameterPropertyModifier: i32 = 92;
    pub const NonPublicAccessibilityModifier: i32 = 24;
    pub const TypeScriptModifier: i32 = 2270;
    pub const ExportDefault: i32 = 513;
}
pub struct JsxFlags(i32);
impl JsxFlags {
    pub const None: i32 = 0;
    /** An element from a named property of the JSX.IntrinsicElements interface */
    pub const IntrinsicNamedElement: i32 = 1;
    /** An element inferred from the string index signature of the JSX.IntrinsicElements interface */
    pub const IntrinsicIndexedElement: i32 = 2;
    pub const IntrinsicElement: i32 = 3;
}

pub struct ModuleKind(i32);
impl ModuleKind {
    pub const None: i32 = 0;
    pub const CommonJS: i32 = 1;
    pub const AMD: i32 = 2;
    pub const UMD: i32 = 3;
    pub const System: i32 = 4;
    pub const ES2015: i32 = 5;
    pub const ESNext: i32 = 6;
}
pub struct JsxEmit(i32);
impl JsxEmit {
    pub const None: i32 = 0;
    pub const Preserve: i32 = 1;
    pub const React: i32 = 2;
    pub const ReactNative: i32 = 3;
}

#[derive(PartialEq,Eq)]
pub struct NewLineKind(i32);
impl NewLineKind {
    pub const CarriageReturnLineFeed: NewLineKind = NewLineKind(0);
    pub const LineFeed: NewLineKind = NewLineKind(1);
}
impl From<NewLineKind> for i32 {
    fn from(v: NewLineKind) -> i32 {
        match v {
            NewLineKind::CarriageReturnLineFeed => 0,
            NewLineKind::LineFeed => 1,
            _ => -1,
        }
    }
}

pub trait ScriptKind: GetId {}
impl ScriptKind for IdBox {}

enum ScriptKindEnum {
    Unknown = 0,
    JS = 1,
    JSX = 2,
    TS = 3,
    TSX = 4,
    External = 5,
}

pub trait ScriptKind_TS{
}
impl GetId for ScriptKind_TS {
    fn id(&self) -> i32 { 
        ScriptKindEnum::TS as i32
    }
}
impl ScriptKind_TS for Enum {}
pub fn ScriptKind_TS_new() -> Box<ScriptKind_TS> {
    Box::new(Enum {})
}
impl From<Box<ScriptKind_TS>> for Box<ScriptKind> {
    fn from(v: Box<ScriptKind_TS>) -> Box<ScriptKind> {
        Box::new(IdBox { id: ScriptKind_TS::id(&*v) })
    }
}

pub trait ScriptTarget: GetId {}
impl ScriptTarget for IdBox {}

pub enum ScriptTargetEnum {
    ES3 = 0,
    ES5 = 1,
    ES2015 = 2,
    ES2016 = 3,
    ES2017 = 4,
    ESNext = 5,
}
pub trait ScriptTargetConst {
    const Latest: i32 = 5;
}

pub trait ScriptTarget_Latest {
}
impl GetId for ScriptTarget_Latest {
    fn id(&self) -> i32 { 
        // ScriptTargetConst::Latest
        5
    }
}
impl ScriptTarget_Latest for Enum {}
pub fn ScriptTarget_Latest_new() -> Box<ScriptTarget_Latest> {
    Box::new(Enum {})
}
impl From<Box<ScriptTarget_Latest>> for Box<ScriptTarget> {
    fn from(v: Box<ScriptTarget_Latest>) -> Box<ScriptTarget> {
        Box::new(IdBox { id: ScriptTarget_Latest::id(&*v) })
    }
}

pub struct LanguageVariant(i32);
impl LanguageVariant {
    pub const Standard: i32 = 0;
    pub const JSX: i32 = 1;
}
pub struct WatchDirectoryFlags(i32);
impl WatchDirectoryFlags {
    pub const None: i32 = 0;
    pub const Recursive: i32 = 1;
}

pub struct Extension(i32);
impl Extension {
    pub const Ts: i32 = 0;
    pub const Tsx: i32 = 1;
    pub const Dts: i32 = 2;
    pub const Js: i32 = 3;
    pub const Jsx: i32 = 4;
    pub const LastTypeScriptExtension: i32 = 2;
}

pub struct EmitFlags(i32);
impl EmitFlags {
    pub const SingleLine: i32 = 1;
    pub const AdviseOnEmitNode: i32 = 2;
    pub const NoSubstitution: i32 = 4;
    pub const CapturesThis: i32 = 8;
    pub const NoLeadingSourceMap: i32 = 16;
    pub const NoTrailingSourceMap: i32 = 32;
    pub const NoSourceMap: i32 = 48;
    pub const NoNestedSourceMaps: i32 = 64;
    pub const NoTokenLeadingSourceMaps: i32 = 128;
    pub const NoTokenTrailingSourceMaps: i32 = 256;
    pub const NoTokenSourceMaps: i32 = 384;
    pub const NoLeadingComments: i32 = 512;
    pub const NoTrailingComments: i32 = 1024;
    pub const NoComments: i32 = 1536;
    pub const NoNestedComments: i32 = 2048;
    pub const HelperName: i32 = 4096;
    pub const ExportName: i32 = 8192;
    pub const LocalName: i32 = 16384;
    pub const InternalName: i32 = 32768;
    pub const Indented: i32 = 65536;
    pub const NoIndentation: i32 = 131072;
    pub const AsyncFunctionBody: i32 = 262144;
    pub const ReuseTempVariableScope: i32 = 524288;
    pub const CustomPrologue: i32 = 1048576;
    pub const NoHoisting: i32 = 2097152;
    pub const HasEndOfDeclarationMarker: i32 = 4194304;
    pub const Iterator: i32 = 8388608;
    pub const NoAsciiEscaping: i32 = 16777216;
}

pub trait EmitHint: GetId {}
impl EmitHint for IdBox {}

pub enum EmitHintEnum {
    SourceFile = 0,
    Expression = 1,
    IdentifierName = 2,
    MappedTypeParameter = 3,
    Unspecified = 4,
}

pub trait EmitHint_Unspecified {}
impl GetId for EmitHint_Unspecified {
    fn id(&self) -> i32 { 
        EmitHintEnum::Unspecified as i32
    }
}
impl EmitHint_Unspecified for Enum {}
pub fn EmitHint_Unspecified_new() -> Box<EmitHint_Unspecified> {
    Box::new(Enum {})
}
impl From<Box<EmitHint_Unspecified>> for Box<EmitHint> {
    fn from(v: Box<EmitHint_Unspecified>) -> Box<EmitHint> {
        Box::new(IdBox { id: EmitHint_Unspecified::id(&*v) })
    }
}

pub trait Statement: GetObject {}
impl Statement for ObjectBox {}

pub trait IfStatement: GetObject {}
impl IfStatement for ObjectBox {}

pub trait Block: GetObject {}
impl Block for ObjectBox {}

pub trait ReturnStatement: GetObject {}
impl ReturnStatement for ObjectBox {}

pub trait TypeNode: GetObject {}
impl TypeNode for ObjectBox {}

pub trait CallExpression: GetObject {}
impl CallExpression for ObjectBox {}

pub trait KeywordTypeNode: GetObject {}
impl KeywordTypeNode for ObjectBox {}

pub trait Token: GetObject {}
impl Token for ObjectBox {}

pub trait FunctionDeclaration: GetObject {}
impl FunctionDeclaration for ObjectBox {}

pub trait TypeParameterDeclaration: GetObject {}
impl TypeParameterDeclaration for ObjectBox {}

pub trait AsteriskToken: GetObject {}
impl AsteriskToken for ObjectBox {}

pub trait Modifier: GetObject {}
impl Modifier for ObjectBox {}

pub trait Decorator: GetObject {}
impl Decorator for ObjectBox {}

impl From<Box<Identifier>> for Box<Node> {
    fn from(v: Box<Identifier>) -> Box<Node> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<Identifier>> for Box<Expression> {
    fn from(v: Box<Identifier>) -> Box<Expression> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<BinaryExpression>> for Box<Node> {
    fn from(v: Box<BinaryExpression>) -> Box<Node> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<FunctionDeclaration>> for Box<Node> {
    fn from(v: Box<FunctionDeclaration>) -> Box<Node> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<Block>> for Box<Node> {
    fn from(v: Box<Block>) -> Box<Node> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<Block>> for Box<Statement> {
    fn from(v: Box<Block>) -> Box<Statement> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<IfStatement>> for Box<Statement> {
    fn from(v: Box<IfStatement>) -> Box<Statement> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<ReturnStatement>> for Box<Statement> {
    fn from(v: Box<ReturnStatement>) -> Box<Statement> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<CallExpression>> for Box<Expression> {
    fn from(v: Box<CallExpression>) -> Box<Expression> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<BinaryExpression>> for Box<Expression> {
    fn from(v: Box<BinaryExpression>) -> Box<Expression> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}

impl From<Box<KeywordTypeNode>> for Box<TypeNode> {
    fn from(v: Box<KeywordTypeNode>) -> Box<TypeNode> {
        Box::new(ObjectBox { object: chakracore::value::Object::clone(&*v.object())})
    }
}