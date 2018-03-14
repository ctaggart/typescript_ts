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

pub struct Js<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
}

impl<'a> Js<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>) -> Js<'a> {
        let js = read_js();
        chakracore::script::eval(&guard, &js).expect("invalid JavaScript code");
        Js { guard }
    }

    /// get the `ts` variable that exposes the TypeScript module
    pub fn ts(&self) -> TsMod {
        TsMod::new(self.guard)
    }
}

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

pub struct TsMod<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> TsMod<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>) -> TsMod {
        let object = guard.global().get(guard, &chakracore::Property::new(guard, "ts")).into_object().unwrap();
        TsMod { guard, object}
    }

    pub fn version(&self) -> String {
        self.object.get(self.guard, &chakracore::Property::new(self.guard, "version")).to_string(self.guard)
    }

    // createNode(kind: SyntaxKind, pos?: number, end?: number): Node;
    pub fn createNode(&self, kind: SyntaxKind, pos: Option<i32>, end: Option<i32> ) -> Node {
        let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createNode")).into_function().unwrap();
        let rv = function.call_with_this(self.guard, &self.object, &[
            &chakracore::value::Number::new(self.guard, kind as i32).into(),
            &chakracore::value::Number::new(self.guard, pos.unwrap_or(-1)).into(),
            &chakracore::value::Number::new(self.guard, end.unwrap_or(-1)).into(),
        ]);
        let object = rv.unwrap().into_object().unwrap();
        Node::new(self.guard, object)
    }

   // function createParameter(decorators: ReadonlyArray<Decorator> | undefined, modifiers: ReadonlyArray<Modifier> | undefined, dotDotDotToken: DotDotDotToken | undefined, name: string | BindingName, questionToken?: QuestionToken, type?: TypeNode, initializer?: Expression): ParameterDeclaration;
   pub fn createParameter(&self, name: &Identifier) -> ParameterDeclaration {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createParameter")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::undefined(self.guard),
           &chakracore::value::undefined(self.guard),
           &chakracore::value::undefined(self.guard),
           &name.object,
       ]);
       let object = rv.unwrap().into_object().unwrap();
       ParameterDeclaration::new(self.guard, object)
   }

   // function createIdentifier(text: string): Identifier;
   pub fn createIdentifier(&self, text: &str) -> Identifier {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createIdentifier")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::String::new(self.guard, text).into(),
       ]);
       let object = rv.unwrap().into_object().unwrap();
       Identifier::new(self.guard, object)
   }

   // function createLiteral(value: number): NumericLiteral;
   pub fn createLiteral_number(&self, value: i32) -> NumericLiteral {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createLiteral")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::Number::new(self.guard, value).into(),
       ]);
       let object = rv.unwrap().into_object().unwrap();
       NumericLiteral::new(self.guard, object)
   }

   // function createBinary(left: Expression, operator: BinaryOperator | BinaryOperatorToken, right: Expression): BinaryExpression;
   // TODO BinaryOperator is a range of SyntaxKind: FirstBinaryOperator to LastBinaryOperator
   pub fn createBinary(&self, left: &Expression, operator: SyntaxKind, right: &Expression) -> BinaryExpression {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createBinary")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &left.object,
           &chakracore::value::Number::new(self.guard, operator as i32).into(),
           &right.object,
       ]);
       let object = rv.unwrap().into_object().unwrap();
       BinaryExpression::new(self.guard, object)
   }

   // function createBlock(statements: ReadonlyArray<Statement>, multiLine?: boolean): Block;
   pub fn createBlock(&self, statements: &[&Statement], multiLine: bool ) -> Block {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createBlock")).into_function().unwrap();

       let statements_length = statements.len() as u32;
       let statements_array = chakracore::value::Array::new(self.guard, statements_length);
       for i in 0..statements_length {
           statements_array.set_index(self.guard, i, &statements[i as usize].object);
       }

       let rv = function.call_with_this(self.guard, &self.object, &[
           &statements_array,
           &chakracore::value::Boolean::new(self.guard, multiLine).into(),
       ]);
       let object = rv.unwrap().into_object().unwrap();
       Block::new(self.guard, object)
   }

   // function createSourceFile(fileName: string, sourceText: string, languageVersion: ScriptTarget, setParentNodes?: boolean, scriptKind?: ScriptKind): SourceFile;
   // ts.createSourceFile("someFileName.ts", "", ts::ScriptTarget::Latest, /*setParentNodes*/ false, ts::ScriptKind::TS);
   pub fn createSourceFile(&self, fileName: &str, sourceText: &str, languageVersion: ScriptTarget, setParentNodes: bool, scriptKind: ScriptKind) -> SourceFile {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createSourceFile")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::String::new(self.guard, fileName).into(),
           &chakracore::value::String::new(self.guard, sourceText).into(),
           &chakracore::value::Number::new(self.guard, languageVersion as i32).into(),
           &chakracore::value::Boolean::new(self.guard, setParentNodes).into(),
           &chakracore::value::Number::new(self.guard, scriptKind as i32).into(),
       ]);
       let object = rv.unwrap().into_object().unwrap();
       SourceFile::new(self.guard, object)
   }

   // function createPrinter(printerOptions?: PrinterOptions, handlers?: PrintHandlers): Printer;
   pub fn createPrinter(&self, printerOptions: &PrinterOptions) -> Printer {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createPrinter")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &printerOptions.object
       ]);
       let object = rv.unwrap().into_object().unwrap();
       Printer::new(self.guard, object)
   }

   // function createReturn(expression?: Expression): ReturnStatement;
   pub fn createReturn(&self, expression: &Expression) -> ReturnStatement {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createReturn")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &expression.object,
       ]);
       let object = rv.unwrap().into_object().unwrap();
       ReturnStatement::new(self.guard, object)
   }

   // function createCall(expression: Expression, typeArguments: ReadonlyArray<TypeNode> | undefined, argumentsArray: ReadonlyArray<Expression>): CallExpression;
   pub fn createCall(&self, expression: &Expression, typeArguments: Option<&[&TypeNode]>, argumentsArray: &[&Expression]) -> CallExpression {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createCall")).into_function().unwrap();

       let argumentsArray_length = argumentsArray.len() as u32;
       let argumentsArray_array = chakracore::value::Array::new(self.guard, argumentsArray_length);
       for i in 0..argumentsArray_length {
           argumentsArray_array.set_index(self.guard, i, &argumentsArray[i as usize].object);
       }

       let rv = function.call_with_this(self.guard, &self.object, &[
           &expression.object,
           &chakracore::value::undefined(self.guard), // TODO typeArguments
           &argumentsArray_array,
       ]);
       let object = rv.unwrap().into_object().unwrap();
       CallExpression::new(self.guard, object)
   }

   // function createIf(expression: Expression, thenStatement: Statement, elseStatement?: Statement): IfStatement;
   pub fn createIf(&self, expression: &Expression, thenStatement: &Statement) -> IfStatement {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createIf")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &expression.object,
           &thenStatement.object,
       ]);
       let object = rv.unwrap().into_object().unwrap();
       IfStatement::new(self.guard, object)
   }

   // function createKeywordTypeNode(kind: KeywordTypeNode["kind"]): KeywordTypeNode;
   pub fn createKeywordTypeNode(&self, kind: SyntaxKind) -> KeywordTypeNode {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createKeywordTypeNode")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::Number::new(self.guard, kind as i32).into(),
       ]);
       let object = rv.unwrap().into_object().unwrap();
       KeywordTypeNode::new(self.guard, object)
   }

   // function createToken<TKind extends SyntaxKind>(token: TKind): Token<TKind>;
   pub fn createToken(&self, token: SyntaxKind) -> Token {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createToken")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::Number::new(self.guard, token as i32).into(),
       ]);
       let object = rv.unwrap().into_object().unwrap();
       Token::new(self.guard, object)
   }

   // function createFunctionDeclaration(decorators: ReadonlyArray<Decorator> | undefined, modifiers: ReadonlyArray<Modifier> | undefined, asteriskToken: AsteriskToken | undefined, name: string | Identifier | undefined, typeParameters: ReadonlyArray<TypeParameterDeclaration> | undefined, parameters: ReadonlyArray<ParameterDeclaration>, type: TypeNode | undefined, body: Block | undefined): FunctionDeclaration;
   pub fn createFunctionDeclaration(&self, decorators: Option<&[&Decorator]>, modifiers: Option<&[&Token]>, asteriskToken: Option<&AsteriskToken>, name: Option<&Identifier>, typeParameters: Option<&[&TypeParameterDeclaration]>, parameters: &[&ParameterDeclaration], type_: Option<&TypeNode>, body: Option<&Block>) -> FunctionDeclaration {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createFunctionDeclaration")).into_function().unwrap();

       let modifiers = modifiers.unwrap();
       let modifiers_length = modifiers.len() as u32;
       let modifiers_array = chakracore::value::Array::new(self.guard, modifiers_length);
       for i in 0..modifiers_length {
           modifiers_array.set_index(self.guard, i, &modifiers[i as usize].object);
       }

       let parameters_length = parameters.len() as u32;
       let parameters_array = chakracore::value::Array::new(self.guard, parameters_length);
       for i in 0..parameters_length {
           parameters_array.set_index(self.guard, i, &parameters[i as usize].object);
       }

       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::undefined(self.guard), // TODO decorators
           &modifiers_array,
           &chakracore::value::undefined(self.guard), // TODO asteriskToken
           &name.unwrap().object,
           &chakracore::value::undefined(self.guard), // TODO typeParameters
           &parameters_array,
           &type_.unwrap().object,
           &body.unwrap().object,
       ]);
       let object = rv.unwrap().into_object().unwrap();
       FunctionDeclaration::new(self.guard, object)
   }

}

pub struct Node<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Node<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Node {
        Node { guard, object}
    }
    pub fn as_TextRange(&self) -> TextRange {
        TextRange::new(self.guard, self.object.clone())
    }

    pub fn kind(&self) -> i32 {
        let kind = self.object.get(self.guard, &chakracore::Property::new(self.guard, "kind"));
        kind.into_number().unwrap().value()
    }
}

pub struct TextRange<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> TextRange<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> TextRange {
        TextRange { guard, object }
    }

   pub fn pos(&self) -> i32 {
       let kind = self.object.get(self.guard, &chakracore::Property::new(self.guard, "pos"));
       kind.into_number().unwrap().value()
   }
   pub fn end(&self) -> i32 {
       let kind = self.object.get(self.guard, &chakracore::Property::new(self.guard, "end"));
       kind.into_number().unwrap().value()
   }
}

//pub trait Declaration: Node {}
pub struct Declaration<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Declaration<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Declaration {
        Declaration { guard, object }
    }
}

pub struct SourceFile<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> SourceFile<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> SourceFile {
        SourceFile { guard, object }
    }

   fn fileName(&self) -> String {
       self.object.get(self.guard, &chakracore::Property::new(self.guard, "fileName")).into_string().unwrap().value()
   }
}


pub struct Expression<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Expression<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Expression {
        Expression { guard, object }
    }
}

pub struct NumericLiteral<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> NumericLiteral<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> NumericLiteral {
        NumericLiteral { guard, object }
    }

    pub fn as_Expression(&self) -> Expression {
        Expression::new(self.guard, self.object.clone())
    }
}

pub struct ParameterDeclaration<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> ParameterDeclaration<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> ParameterDeclaration {
        ParameterDeclaration { guard, object}
    }
}

pub struct PrinterOptions<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> PrinterOptions<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> PrinterOptions {
        PrinterOptions { guard, object}
    }

   pub fn set_newLine(&self, value: Option<NewLineKind>) {
       let property = &chakracore::Property::new(self.guard, "newLine");
       match value {
           None => {
               let jsv = &chakracore::value::undefined(self.guard);
               self.object.set(self.guard, property, jsv);
           },
           Some(v) => {
               let jsv = &chakracore::value::Number::new(self.guard, i32::from(v));
               self.object.set(self.guard, property, jsv);
           },
       }
   }
}

pub struct Printer<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Printer<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Printer {
        Printer { guard, object}
    }

   // printNode(hint: EmitHint, node: Node, sourceFile: SourceFile): string;
   pub fn printNode(&self, hint: EmitHint, node: &Node, sourceFile: &SourceFile) -> String {
       let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "printNode")).into_function().unwrap();
       let rv = function.call_with_this(self.guard, &self.object, &[
           &chakracore::value::Number::new(self.guard, hint as i32).into(),
           &node.object,
           &sourceFile.object,
       ]);
       rv.unwrap().into_string().unwrap().value()
   }
}

pub struct BinaryOperator<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> BinaryOperator<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> BinaryOperator {
        BinaryOperator { guard, object }
    }
}

pub enum SyntaxKind {
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

pub enum NodeFlags {
   None = 0,
   Let = 1,
   Const = 2,
   NestedNamespace = 4,
   Synthesized = 8,
   Namespace = 16,
   ExportContext = 32,
   ContainsThis = 64,
   HasImplicitReturn = 128,
   HasExplicitReturn = 256,
   GlobalAugmentation = 512,
   HasAsyncFunctions = 1024,
   DisallowInContext = 2048,
   YieldContext = 4096,
   DecoratorContext = 8192,
   AwaitContext = 16384,
   ThisNodeHasError = 32768,
   JavaScriptFile = 65536,
   ThisNodeOrAnySubNodesHasError = 131072,
   HasAggregatedChildData = 262144,
   BlockScoped = 3,
   ReachabilityCheckFlags = 384,
   ReachabilityAndEmitFlags = 1408,
   ContextFlags = 96256,
   TypeExcludesFlags = 20480,
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

pub enum ScriptKind {
   Unknown = 0,
   JS = 1,
   JSX = 2,
   TS = 3,
   TSX = 4,
   External = 5,
}

pub enum ScriptTarget {
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

pub enum EmitHint {
   SourceFile = 0,
   Expression = 1,
   IdentifierName = 2,
   MappedTypeParameter = 3,
   Unspecified = 4,
}

pub struct Statement<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Statement<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Statement {
        Statement { guard, object }
    }
}

pub struct IfStatement<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> IfStatement<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> IfStatement {
        IfStatement { guard, object }
    }
    pub fn as_Statement(&self) -> Statement {
        Statement::new(self.guard, self.object.clone())
    }
}

pub struct Block<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Block<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Block {
        Block { guard, object }
    }
    pub fn as_Statement(&self) -> Statement {
        Statement::new(self.guard, self.object.clone())
    }
}

pub struct ReturnStatement<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> ReturnStatement<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> ReturnStatement {
        ReturnStatement { guard, object }
    }
    pub fn as_Statement(&self) -> Statement {
        Statement::new(self.guard, self.object.clone())
    }
}

pub struct TypeNode<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> TypeNode<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> TypeNode {
        TypeNode { guard, object }
    }
}

pub struct CallExpression<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> CallExpression<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> CallExpression {
        CallExpression { guard, object }
    }
    pub fn as_Expression(&self) -> Expression {
        Expression::new(self.guard, self.object.clone())
    }
}

pub struct KeywordTypeNode<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> KeywordTypeNode<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> KeywordTypeNode {
        KeywordTypeNode { guard, object }
    }
    pub fn as_TypeNode(&self) -> TypeNode {
        TypeNode::new(self.guard, self.object.clone())
    }
}

pub struct Token<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Token<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Token {
        Token { guard, object }
    }
}

pub struct FunctionDeclaration<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> FunctionDeclaration<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> FunctionDeclaration {
        FunctionDeclaration { guard, object }
    }
    pub fn as_Node(&self) -> Node {
        Node::new(self.guard, self.object.clone())
    }
}

pub struct TypeParameterDeclaration {}

pub struct AsteriskToken {}

pub struct Modifier {}

pub struct Decorator {}

pub struct Identifier<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> Identifier<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> Identifier {
        Identifier { guard, object }
    }
    pub fn as_Expression(&self) -> Expression {
        Expression::new(self.guard, self.object.clone())
    }
}

pub struct BinaryExpression<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
    object: chakracore::value::Object
}
impl<'a> BinaryExpression<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>, object: chakracore::value::Object) -> BinaryExpression {
        BinaryExpression { guard, object }
    }
    pub fn as_Expression(&self) -> Expression {
        Expression::new(self.guard, self.object.clone())
    }
}