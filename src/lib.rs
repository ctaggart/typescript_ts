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
    // fn object_mut(&mut self) -> &mut chakracore::value::Object;
}

pub struct ObjectBox {
    object: chakracore::value::Object,
}

impl ObjectBox {
    pub fn new(guard: &chakracore::context::ContextGuard) -> ObjectBox {
        ObjectBox { object: chakracore::value::Object::new(guard) }
    }
    // pub fn new_box(guard: &chakracore::context::ContextGuard) -> Box<ObjectBox> {
    //     Box::new(ObjectBox::new(guard))
    // }
}

impl GetObject for ObjectBox {
    fn object(&self) -> &chakracore::value::Object {
        &self.object
    }
    // fn object_mut(&mut self) -> &mut chakracore::value::Object {
    //     &mut self.object
    // }
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
    fn createNode(&self, guard: &chakracore::context::ContextGuard, kind: SyntaxKind, pos: Option<i32>, end: Option<i32> ) -> Box<Node> {
        let SyntaxKind(kind_) = kind; 
        let tsmod = self.object();
        let function = tsmod.get(guard, &chakracore::Property::new(guard, "createNode")).into_function().unwrap();
        let rv = function.call_with_this(guard, tsmod, &[
            &chakracore::value::Number::new(guard, kind_).into(),
            &chakracore::value::Number::new(guard, pos.unwrap_or(-1)).into(),
            &chakracore::value::Number::new(guard, end.unwrap_or(-1)).into(),
        ]);
        let node = rv.unwrap().into_object().unwrap();
        Box::new(ObjectBox { object: node })
    }

    // function createParameter(decorators: ReadonlyArray<Decorator> | undefined, modifiers: ReadonlyArray<Modifier> | undefined, dotDotDotToken: DotDotDotToken | undefined, name: string | BindingName, questionToken?: QuestionToken, type?: TypeNode, initializer?: Expression): ParameterDeclaration;
    fn createParameter(&self, guard: &chakracore::context::ContextGuard, name: &Identifier) -> Box<ParameterDeclaration> {
        let tsmod = self.object();
        let function = tsmod.get(guard, &chakracore::Property::new(guard, "createParameter")).into_function().unwrap();
        let rv = function.call_with_this(guard, tsmod, &[
            &chakracore::value::undefined(guard),
            &chakracore::value::undefined(guard),
            &chakracore::value::undefined(guard),
            name.object(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createIdentifier(text: string): Identifier;
    fn createIdentifier(&self, guard: &chakracore::context::ContextGuard, text: &str) -> Box<Identifier> {
        let tsmod = self.object();
        let function = tsmod.get(guard, &chakracore::Property::new(guard, "createIdentifier")).into_function().unwrap();
        let rv = function.call_with_this(guard, tsmod, &[
            &chakracore::value::String::new(guard, text).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createLiteral(value: number): NumericLiteral;
    fn createLiteral_number(&self, guard: &chakracore::context::ContextGuard, value: i32) -> Box<NumericLiteral> {
        let tsmod = self.object();
        let function = tsmod.get(guard, &chakracore::Property::new(guard, "createLiteral")).into_function().unwrap();
        let rv = function.call_with_this(guard, tsmod, &[
            &chakracore::value::Number::new(guard, value).into(),
        ]);
        Box::new(ObjectBox { object: rv.unwrap().into_object().unwrap() })
    }

    // function createBinary(left: Expression, operator: BinaryOperator | BinaryOperatorToken, right: Expression): BinaryExpression;
    fn createBinary(&self, guard: &chakracore::context::ContextGuard, left: &Expression, operator: &BinaryOperatorToken, right: &Expression) -> Box<BinaryExpression> {
        let tsmod = self.object();
        let function = tsmod.get(guard, &chakracore::Property::new(guard, "createBinary")).into_function().unwrap();
        let rv = function.call_with_this(guard, tsmod, &[
            left.object(),
            operator.object(),
            right.object(),
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

pub trait TextRange {

}

impl TextRange for ObjectBox {}

pub trait Declaration: Node {

}

impl Declaration for ObjectBox {}

pub trait SourceFile: Declaration {

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


    // interface Identifier extends PrimaryExpression {
    //     kind: SyntaxKind.Identifier;

pub trait Identifier: GetObject + PrimaryExpression + AsExpression { 
}

impl Identifier for ObjectBox {}

    // interface PrinterOptions {
    //     removeComments?: boolean;
    //     newLine?: NewLineKind;
    // }

pub trait NumericLiteral: GetObject {

}

impl NumericLiteral for ObjectBox {}

pub trait ParameterDeclaration {

}

impl ParameterDeclaration for ObjectBox {}

pub trait PrinterOptions : GetObject  {
    // fn set_newLine(&mut self, guard: &chakracore::context::ContextGuard, value: Option<NewLineKind>) {
    //     let property = &chakracore::Property::new(guard, "newLine");
    //     match value {
    //         None => {
    //             let jsv = &chakracore::value::undefined(guard);
    //             self.object_mut().set(guard, property, jsv);
    //         },
    //         Some(v) => {
    //             let jsv = &chakracore::value::Number::new(guard, i32::from(v));
    //             self.object_mut().set(guard, property, jsv);
    //         },
    //     }
    // }
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


// TODO was limited to SyntaxKind, but that needs to be a trait
pub trait Token<TKind> {}
// impl Token for ObjectBox {}

//  type BinaryOperator = AssignmentOperatorOrHigher | SyntaxKind.CommaToken;
pub trait BinaryOperator: GetObject {}
impl BinaryOperator for ObjectBox {}

// pub trait BinaryOperatorToken: Token<BinaryOperator> {}
// TODO BinaryOperator + 'static` does not have a constant size known at compile-time
pub trait BinaryOperatorToken: GetObject {}
impl BinaryOperatorToken for ObjectBox {}


// interface BinaryExpression extends Expression, Declaration {
//     kind: SyntaxKind.BinaryExpression;
//     left: Expression;
//     operatorToken: BinaryOperatorToken;
//     right: Expression;
// }
pub trait BinaryExpression: Expression + Declaration {}
impl BinaryExpression for ObjectBox {}

pub struct SyntaxKind(i32);
impl SyntaxKind {
    pub const Unknown: SyntaxKind = SyntaxKind(0);
    pub const EndOfFileToken: SyntaxKind = SyntaxKind(1);
    pub const SingleLineCommentTrivia: SyntaxKind = SyntaxKind(2);
    pub const MultiLineCommentTrivia: SyntaxKind = SyntaxKind(3);
    pub const NewLineTrivia: SyntaxKind = SyntaxKind(4);
    pub const WhitespaceTrivia: SyntaxKind = SyntaxKind(5);
    pub const ShebangTrivia: SyntaxKind = SyntaxKind(6);
    pub const ConflictMarkerTrivia: SyntaxKind = SyntaxKind(7);
    pub const NumericLiteral: SyntaxKind = SyntaxKind(8);
    pub const StringLiteral: SyntaxKind = SyntaxKind(9);
    pub const JsxText: SyntaxKind = SyntaxKind(10);
    pub const JsxTextAllWhiteSpaces: SyntaxKind = SyntaxKind(11);
    pub const RegularExpressionLiteral: SyntaxKind = SyntaxKind(12);
    pub const NoSubstitutionTemplateLiteral: SyntaxKind = SyntaxKind(13);
    pub const TemplateHead: SyntaxKind = SyntaxKind(14);
    pub const TemplateMiddle: SyntaxKind = SyntaxKind(15);
    pub const TemplateTail: SyntaxKind = SyntaxKind(16);
    pub const OpenBraceToken: SyntaxKind = SyntaxKind(17);
    pub const CloseBraceToken: SyntaxKind = SyntaxKind(18);
    pub const OpenParenToken: SyntaxKind = SyntaxKind(19);
    pub const CloseParenToken: SyntaxKind = SyntaxKind(20);
    pub const OpenBracketToken: SyntaxKind = SyntaxKind(21);
    pub const CloseBracketToken: SyntaxKind = SyntaxKind(22);
    pub const DotToken: SyntaxKind = SyntaxKind(23);
    pub const DotDotDotToken: SyntaxKind = SyntaxKind(24);
    pub const SemicolonToken: SyntaxKind = SyntaxKind(25);
    pub const CommaToken: SyntaxKind = SyntaxKind(26);
    pub const LessThanToken: SyntaxKind = SyntaxKind(27);
    pub const LessThanSlashToken: SyntaxKind = SyntaxKind(28);
    pub const GreaterThanToken: SyntaxKind = SyntaxKind(29);
    pub const LessThanEqualsToken: SyntaxKind = SyntaxKind(30);
    pub const GreaterThanEqualsToken: SyntaxKind = SyntaxKind(31);
    pub const EqualsEqualsToken: SyntaxKind = SyntaxKind(32);
    pub const ExclamationEqualsToken: SyntaxKind = SyntaxKind(33);
    pub const EqualsEqualsEqualsToken: SyntaxKind = SyntaxKind(34);
    pub const ExclamationEqualsEqualsToken: SyntaxKind = SyntaxKind(35);
    pub const EqualsGreaterThanToken: SyntaxKind = SyntaxKind(36);
    pub const PlusToken: SyntaxKind = SyntaxKind(37);
    pub const MinusToken: SyntaxKind = SyntaxKind(38);
    pub const AsteriskToken: SyntaxKind = SyntaxKind(39);
    pub const AsteriskAsteriskToken: SyntaxKind = SyntaxKind(40);
    pub const SlashToken: SyntaxKind = SyntaxKind(41);
    pub const PercentToken: SyntaxKind = SyntaxKind(42);
    pub const PlusPlusToken: SyntaxKind = SyntaxKind(43);
    pub const MinusMinusToken: SyntaxKind = SyntaxKind(44);
    pub const LessThanLessThanToken: SyntaxKind = SyntaxKind(45);
    pub const GreaterThanGreaterThanToken: SyntaxKind = SyntaxKind(46);
    pub const GreaterThanGreaterThanGreaterThanToken: SyntaxKind = SyntaxKind(47);
    pub const AmpersandToken: SyntaxKind = SyntaxKind(48);
    pub const BarToken: SyntaxKind = SyntaxKind(49);
    pub const CaretToken: SyntaxKind = SyntaxKind(50);
    pub const ExclamationToken: SyntaxKind = SyntaxKind(51);
    pub const TildeToken: SyntaxKind = SyntaxKind(52);
    pub const AmpersandAmpersandToken: SyntaxKind = SyntaxKind(53);
    pub const BarBarToken: SyntaxKind = SyntaxKind(54);
    pub const QuestionToken: SyntaxKind = SyntaxKind(55);
    pub const ColonToken: SyntaxKind = SyntaxKind(56);
    pub const AtToken: SyntaxKind = SyntaxKind(57);
    pub const EqualsToken: SyntaxKind = SyntaxKind(58);
    pub const PlusEqualsToken: SyntaxKind = SyntaxKind(59);
    pub const MinusEqualsToken: SyntaxKind = SyntaxKind(60);
    pub const AsteriskEqualsToken: SyntaxKind = SyntaxKind(61);
    pub const AsteriskAsteriskEqualsToken: SyntaxKind = SyntaxKind(62);
    pub const SlashEqualsToken: SyntaxKind = SyntaxKind(63);
    pub const PercentEqualsToken: SyntaxKind = SyntaxKind(64);
    pub const LessThanLessThanEqualsToken: SyntaxKind = SyntaxKind(65);
    pub const GreaterThanGreaterThanEqualsToken: SyntaxKind = SyntaxKind(66);
    pub const GreaterThanGreaterThanGreaterThanEqualsToken: SyntaxKind = SyntaxKind(67);
    pub const AmpersandEqualsToken: SyntaxKind = SyntaxKind(68);
    pub const BarEqualsToken: SyntaxKind = SyntaxKind(69);
    pub const CaretEqualsToken: SyntaxKind = SyntaxKind(70);
    pub const Identifier: SyntaxKind = SyntaxKind(71);
    pub const BreakKeyword: SyntaxKind = SyntaxKind(72);
    pub const CaseKeyword: SyntaxKind = SyntaxKind(73);
    pub const CatchKeyword: SyntaxKind = SyntaxKind(74);
    pub const ClassKeyword: SyntaxKind = SyntaxKind(75);
    pub const ConstKeyword: SyntaxKind = SyntaxKind(76);
    pub const ContinueKeyword: SyntaxKind = SyntaxKind(77);
    pub const DebuggerKeyword: SyntaxKind = SyntaxKind(78);
    pub const DefaultKeyword: SyntaxKind = SyntaxKind(79);
    pub const DeleteKeyword: SyntaxKind = SyntaxKind(80);
    pub const DoKeyword: SyntaxKind = SyntaxKind(81);
    pub const ElseKeyword: SyntaxKind = SyntaxKind(82);
    pub const traitKeyword: SyntaxKind = SyntaxKind(83);
    pub const ExportKeyword: SyntaxKind = SyntaxKind(84);
    pub const ExtendsKeyword: SyntaxKind = SyntaxKind(85);
    pub const FalseKeyword: SyntaxKind = SyntaxKind(86);
    pub const FinallyKeyword: SyntaxKind = SyntaxKind(87);
    pub const ForKeyword: SyntaxKind = SyntaxKind(88);
    pub const FunctionKeyword: SyntaxKind = SyntaxKind(89);
    pub const IfKeyword: SyntaxKind = SyntaxKind(90);
    pub const ImportKeyword: SyntaxKind = SyntaxKind(91);
    pub const InKeyword: SyntaxKind = SyntaxKind(92);
    pub const InstanceOfKeyword: SyntaxKind = SyntaxKind(93);
    pub const NewKeyword: SyntaxKind = SyntaxKind(94);
    pub const NullKeyword: SyntaxKind = SyntaxKind(95);
    pub const ReturnKeyword: SyntaxKind = SyntaxKind(96);
    pub const SuperKeyword: SyntaxKind = SyntaxKind(97);
    pub const SwitchKeyword: SyntaxKind = SyntaxKind(98);
    pub const ThisKeyword: SyntaxKind = SyntaxKind(99);
    pub const ThrowKeyword: SyntaxKind = SyntaxKind(100);
    pub const TrueKeyword: SyntaxKind = SyntaxKind(101);
    pub const TryKeyword: SyntaxKind = SyntaxKind(102);
    pub const TypeOfKeyword: SyntaxKind = SyntaxKind(103);
    pub const VarKeyword: SyntaxKind = SyntaxKind(104);
    pub const VoidKeyword: SyntaxKind = SyntaxKind(105);
    pub const WhileKeyword: SyntaxKind = SyntaxKind(106);
    pub const WithKeyword: SyntaxKind = SyntaxKind(107);
    pub const ImplementsKeyword: SyntaxKind = SyntaxKind(108);
    pub const InterfaceKeyword: SyntaxKind = SyntaxKind(109);
    pub const LetKeyword: SyntaxKind = SyntaxKind(110);
    pub const PackageKeyword: SyntaxKind = SyntaxKind(111);
    pub const PrivateKeyword: SyntaxKind = SyntaxKind(112);
    pub const ProtectedKeyword: SyntaxKind = SyntaxKind(113);
    pub const PublicKeyword: SyntaxKind = SyntaxKind(114);
    pub const StaticKeyword: SyntaxKind = SyntaxKind(115);
    pub const YieldKeyword: SyntaxKind = SyntaxKind(116);
    pub const AbstractKeyword: SyntaxKind = SyntaxKind(117);
    pub const AsKeyword: SyntaxKind = SyntaxKind(118);
    pub const AnyKeyword: SyntaxKind = SyntaxKind(119);
    pub const AsyncKeyword: SyntaxKind = SyntaxKind(120);
    pub const AwaitKeyword: SyntaxKind = SyntaxKind(121);
    pub const BooleanKeyword: SyntaxKind = SyntaxKind(122);
    pub const ConstructorKeyword: SyntaxKind = SyntaxKind(123);
    pub const DeclareKeyword: SyntaxKind = SyntaxKind(124);
    pub const GetKeyword: SyntaxKind = SyntaxKind(125);
    pub const IsKeyword: SyntaxKind = SyntaxKind(126);
    pub const KeyOfKeyword: SyntaxKind = SyntaxKind(127);
    pub const ModuleKeyword: SyntaxKind = SyntaxKind(128);
    pub const NamespaceKeyword: SyntaxKind = SyntaxKind(129);
    pub const NeverKeyword: SyntaxKind = SyntaxKind(130);
    pub const ReadonlyKeyword: SyntaxKind = SyntaxKind(131);
    pub const RequireKeyword: SyntaxKind = SyntaxKind(132);
    pub const NumberKeyword: SyntaxKind = SyntaxKind(133);
    pub const ObjectKeyword: SyntaxKind = SyntaxKind(134);
    pub const SetKeyword: SyntaxKind = SyntaxKind(135);
    pub const StringKeyword: SyntaxKind = SyntaxKind(136);
    pub const SymbolKeyword: SyntaxKind = SyntaxKind(137);
    pub const TypeKeyword: SyntaxKind = SyntaxKind(138);
    pub const UndefinedKeyword: SyntaxKind = SyntaxKind(139);
    pub const FromKeyword: SyntaxKind = SyntaxKind(140);
    pub const GlobalKeyword: SyntaxKind = SyntaxKind(141);
    pub const OfKeyword: SyntaxKind = SyntaxKind(142);
    pub const QualifiedName: SyntaxKind = SyntaxKind(143);
    pub const ComputedPropertyName: SyntaxKind = SyntaxKind(144);
    pub const TypeParameter: SyntaxKind = SyntaxKind(145);
    pub const Parameter: SyntaxKind = SyntaxKind(146);
    pub const Decorator: SyntaxKind = SyntaxKind(147);
    pub const PropertySignature: SyntaxKind = SyntaxKind(148);
    pub const PropertyDeclaration: SyntaxKind = SyntaxKind(149);
    pub const MethodSignature: SyntaxKind = SyntaxKind(150);
    pub const MethodDeclaration: SyntaxKind = SyntaxKind(151);
    pub const Constructor: SyntaxKind = SyntaxKind(152);
    pub const GetAccessor: SyntaxKind = SyntaxKind(153);
    pub const SetAccessor: SyntaxKind = SyntaxKind(154);
    pub const CallSignature: SyntaxKind = SyntaxKind(155);
    pub const ConstructSignature: SyntaxKind = SyntaxKind(156);
    pub const IndexSignature: SyntaxKind = SyntaxKind(157);
    pub const TypePredicate: SyntaxKind = SyntaxKind(158);
    pub const TypeReference: SyntaxKind = SyntaxKind(159);
    pub const FunctionType: SyntaxKind = SyntaxKind(160);
    pub const ConstructorType: SyntaxKind = SyntaxKind(161);
    pub const TypeQuery: SyntaxKind = SyntaxKind(162);
    pub const TypeLiteral: SyntaxKind = SyntaxKind(163);
    pub const ArrayType: SyntaxKind = SyntaxKind(164);
    pub const TupleType: SyntaxKind = SyntaxKind(165);
    pub const UnionType: SyntaxKind = SyntaxKind(166);
    pub const IntersectionType: SyntaxKind = SyntaxKind(167);
    pub const ParenthesizedType: SyntaxKind = SyntaxKind(168);
    pub const ThisType: SyntaxKind = SyntaxKind(169);
    pub const TypeOperator: SyntaxKind = SyntaxKind(170);
    pub const IndexedAccessType: SyntaxKind = SyntaxKind(171);
    pub const MappedType: SyntaxKind = SyntaxKind(172);
    pub const LiteralType: SyntaxKind = SyntaxKind(173);
    pub const ObjectBindingPattern: SyntaxKind = SyntaxKind(174);
    pub const ArrayBindingPattern: SyntaxKind = SyntaxKind(175);
    pub const BindingElement: SyntaxKind = SyntaxKind(176);
    pub const ArrayLiteralExpression: SyntaxKind = SyntaxKind(177);
    pub const ObjectLiteralExpression: SyntaxKind = SyntaxKind(178);
    pub const PropertyAccessExpression: SyntaxKind = SyntaxKind(179);
    pub const ElementAccessExpression: SyntaxKind = SyntaxKind(180);
    pub const CallExpression: SyntaxKind = SyntaxKind(181);
    pub const NewExpression: SyntaxKind = SyntaxKind(182);
    pub const TaggedTemplateExpression: SyntaxKind = SyntaxKind(183);
    pub const TypeAssertionExpression: SyntaxKind = SyntaxKind(184);
    pub const ParenthesizedExpression: SyntaxKind = SyntaxKind(185);
    pub const FunctionExpression: SyntaxKind = SyntaxKind(186);
    pub const ArrowFunction: SyntaxKind = SyntaxKind(187);
    pub const DeleteExpression: SyntaxKind = SyntaxKind(188);
    pub const TypeOfExpression: SyntaxKind = SyntaxKind(189);
    pub const VoidExpression: SyntaxKind = SyntaxKind(190);
    pub const AwaitExpression: SyntaxKind = SyntaxKind(191);
    pub const PrefixUnaryExpression: SyntaxKind = SyntaxKind(192);
    pub const PostfixUnaryExpression: SyntaxKind = SyntaxKind(193);
    pub const BinaryExpression: SyntaxKind = SyntaxKind(194);
    pub const ConditionalExpression: SyntaxKind = SyntaxKind(195);
    pub const TemplateExpression: SyntaxKind = SyntaxKind(196);
    pub const YieldExpression: SyntaxKind = SyntaxKind(197);
    pub const SpreadElement: SyntaxKind = SyntaxKind(198);
    pub const ClassExpression: SyntaxKind = SyntaxKind(199);
    pub const OmittedExpression: SyntaxKind = SyntaxKind(200);
    pub const ExpressionWithTypeArguments: SyntaxKind = SyntaxKind(201);
    pub const AsExpression: SyntaxKind = SyntaxKind(202);
    pub const NonNullExpression: SyntaxKind = SyntaxKind(203);
    pub const MetaProperty: SyntaxKind = SyntaxKind(204);
    pub const TemplateSpan: SyntaxKind = SyntaxKind(205);
    pub const SemicolonClassElement: SyntaxKind = SyntaxKind(206);
    pub const Block: SyntaxKind = SyntaxKind(207);
    pub const VariableStatement: SyntaxKind = SyntaxKind(208);
    pub const EmptyStatement: SyntaxKind = SyntaxKind(209);
    pub const ExpressionStatement: SyntaxKind = SyntaxKind(210);
    pub const IfStatement: SyntaxKind = SyntaxKind(211);
    pub const DoStatement: SyntaxKind = SyntaxKind(212);
    pub const WhileStatement: SyntaxKind = SyntaxKind(213);
    pub const ForStatement: SyntaxKind = SyntaxKind(214);
    pub const ForInStatement: SyntaxKind = SyntaxKind(215);
    pub const ForOfStatement: SyntaxKind = SyntaxKind(216);
    pub const ContinueStatement: SyntaxKind = SyntaxKind(217);
    pub const BreakStatement: SyntaxKind = SyntaxKind(218);
    pub const ReturnStatement: SyntaxKind = SyntaxKind(219);
    pub const WithStatement: SyntaxKind = SyntaxKind(220);
    pub const SwitchStatement: SyntaxKind = SyntaxKind(221);
    pub const LabeledStatement: SyntaxKind = SyntaxKind(222);
    pub const ThrowStatement: SyntaxKind = SyntaxKind(223);
    pub const TryStatement: SyntaxKind = SyntaxKind(224);
    pub const DebuggerStatement: SyntaxKind = SyntaxKind(225);
    pub const VariableDeclaration: SyntaxKind = SyntaxKind(226);
    pub const VariableDeclarationList: SyntaxKind = SyntaxKind(227);
    pub const FunctionDeclaration: SyntaxKind = SyntaxKind(228);
    pub const ClassDeclaration: SyntaxKind = SyntaxKind(229);
    pub const InterfaceDeclaration: SyntaxKind = SyntaxKind(230);
    pub const TypeAliasDeclaration: SyntaxKind = SyntaxKind(231);
    pub const traitDeclaration: SyntaxKind = SyntaxKind(232);
    pub const ModuleDeclaration: SyntaxKind = SyntaxKind(233);
    pub const ModuleBlock: SyntaxKind = SyntaxKind(234);
    pub const CaseBlock: SyntaxKind = SyntaxKind(235);
    pub const NamespaceExportDeclaration: SyntaxKind = SyntaxKind(236);
    pub const ImportEqualsDeclaration: SyntaxKind = SyntaxKind(237);
    pub const ImportDeclaration: SyntaxKind = SyntaxKind(238);
    pub const ImportClause: SyntaxKind = SyntaxKind(239);
    pub const NamespaceImport: SyntaxKind = SyntaxKind(240);
    pub const NamedImports: SyntaxKind = SyntaxKind(241);
    pub const ImportSpecifier: SyntaxKind = SyntaxKind(242);
    pub const ExportAssignment: SyntaxKind = SyntaxKind(243);
    pub const ExportDeclaration: SyntaxKind = SyntaxKind(244);
    pub const NamedExports: SyntaxKind = SyntaxKind(245);
    pub const ExportSpecifier: SyntaxKind = SyntaxKind(246);
    pub const MissingDeclaration: SyntaxKind = SyntaxKind(247);
    pub const ExternalModuleReference: SyntaxKind = SyntaxKind(248);
    pub const JsxElement: SyntaxKind = SyntaxKind(249);
    pub const JsxSelfClosingElement: SyntaxKind = SyntaxKind(250);
    pub const JsxOpeningElement: SyntaxKind = SyntaxKind(251);
    pub const JsxClosingElement: SyntaxKind = SyntaxKind(252);
    pub const JsxAttribute: SyntaxKind = SyntaxKind(253);
    pub const JsxAttributes: SyntaxKind = SyntaxKind(254);
    pub const JsxSpreadAttribute: SyntaxKind = SyntaxKind(255);
    pub const JsxExpression: SyntaxKind = SyntaxKind(256);
    pub const CaseClause: SyntaxKind = SyntaxKind(257);
    pub const DefaultClause: SyntaxKind = SyntaxKind(258);
    pub const HeritageClause: SyntaxKind = SyntaxKind(259);
    pub const CatchClause: SyntaxKind = SyntaxKind(260);
    pub const PropertyAssignment: SyntaxKind = SyntaxKind(261);
    pub const ShorthandPropertyAssignment: SyntaxKind = SyntaxKind(262);
    pub const SpreadAssignment: SyntaxKind = SyntaxKind(263);
    pub const traitMember: SyntaxKind = SyntaxKind(264);
    pub const SourceFile: SyntaxKind = SyntaxKind(265);
    pub const Bundle: SyntaxKind = SyntaxKind(266);
    pub const JSDocTypeExpression: SyntaxKind = SyntaxKind(267);
    pub const JSDocAllType: SyntaxKind = SyntaxKind(268);
    pub const JSDocUnknownType: SyntaxKind = SyntaxKind(269);
    pub const JSDocArrayType: SyntaxKind = SyntaxKind(270);
    pub const JSDocUnionType: SyntaxKind = SyntaxKind(271);
    pub const JSDocTupleType: SyntaxKind = SyntaxKind(272);
    pub const JSDocNullableType: SyntaxKind = SyntaxKind(273);
    pub const JSDocNonNullableType: SyntaxKind = SyntaxKind(274);
    pub const JSDocRecordType: SyntaxKind = SyntaxKind(275);
    pub const JSDocRecordMember: SyntaxKind = SyntaxKind(276);
    pub const JSDocTypeReference: SyntaxKind = SyntaxKind(277);
    pub const JSDocOptionalType: SyntaxKind = SyntaxKind(278);
    pub const JSDocFunctionType: SyntaxKind = SyntaxKind(279);
    pub const JSDocVariadicType: SyntaxKind = SyntaxKind(280);
    pub const JSDocConstructorType: SyntaxKind = SyntaxKind(281);
    pub const JSDocThisType: SyntaxKind = SyntaxKind(282);
    pub const JSDocComment: SyntaxKind = SyntaxKind(283);
    pub const JSDocTag: SyntaxKind = SyntaxKind(284);
    pub const JSDocAugmentsTag: SyntaxKind = SyntaxKind(285);
    pub const JSDocClassTag: SyntaxKind = SyntaxKind(286);
    pub const JSDocParameterTag: SyntaxKind = SyntaxKind(287);
    pub const JSDocReturnTag: SyntaxKind = SyntaxKind(288);
    pub const JSDocTypeTag: SyntaxKind = SyntaxKind(289);
    pub const JSDocTemplateTag: SyntaxKind = SyntaxKind(290);
    pub const JSDocTypedefTag: SyntaxKind = SyntaxKind(291);
    pub const JSDocPropertyTag: SyntaxKind = SyntaxKind(292);
    pub const JSDocTypeLiteral: SyntaxKind = SyntaxKind(293);
    pub const JSDocLiteralType: SyntaxKind = SyntaxKind(294);
    pub const SyntaxList: SyntaxKind = SyntaxKind(295);
    pub const NotEmittedStatement: SyntaxKind = SyntaxKind(296);
    pub const PartiallyEmittedExpression: SyntaxKind = SyntaxKind(297);
    pub const CommaListExpression: SyntaxKind = SyntaxKind(298);
    pub const MergeDeclarationMarker: SyntaxKind = SyntaxKind(299);
    pub const EndOfDeclarationMarker: SyntaxKind = SyntaxKind(300);
    pub const Count: SyntaxKind = SyntaxKind(301);
    pub const FirstAssignment: SyntaxKind = SyntaxKind(58);
    pub const LastAssignment: SyntaxKind = SyntaxKind(70);
    pub const FirstCompoundAssignment: SyntaxKind = SyntaxKind(59);
    pub const LastCompoundAssignment: SyntaxKind = SyntaxKind(70);
    pub const FirstReservedWord: SyntaxKind = SyntaxKind(72);
    pub const LastReservedWord: SyntaxKind = SyntaxKind(107);
    pub const FirstKeyword: SyntaxKind = SyntaxKind(72);
    pub const LastKeyword: SyntaxKind = SyntaxKind(142);
    pub const FirstFutureReservedWord: SyntaxKind = SyntaxKind(108);
    pub const LastFutureReservedWord: SyntaxKind = SyntaxKind(116);
    pub const FirstTypeNode: SyntaxKind = SyntaxKind(158);
    pub const LastTypeNode: SyntaxKind = SyntaxKind(173);
    pub const FirstPunctuation: SyntaxKind = SyntaxKind(17);
    pub const LastPunctuation: SyntaxKind = SyntaxKind(70);
    pub const FirstToken: SyntaxKind = SyntaxKind(0);
    pub const LastToken: SyntaxKind = SyntaxKind(142);
    pub const FirstTriviaToken: SyntaxKind = SyntaxKind(2);
    pub const LastTriviaToken: SyntaxKind = SyntaxKind(7);
    pub const FirstLiteralToken: SyntaxKind = SyntaxKind(8);
    pub const LastLiteralToken: SyntaxKind = SyntaxKind(13);
    pub const FirstTemplateToken: SyntaxKind = SyntaxKind(13);
    pub const LastTemplateToken: SyntaxKind = SyntaxKind(16);
    pub const FirstBinaryOperator: SyntaxKind = SyntaxKind(27);
    pub const LastBinaryOperator: SyntaxKind = SyntaxKind(70);
    pub const FirstNode: SyntaxKind = SyntaxKind(143);
    pub const FirstJSDocNode: SyntaxKind = SyntaxKind(267);
    pub const LastJSDocNode: SyntaxKind = SyntaxKind(294);
    pub const FirstJSDocTagNode: SyntaxKind = SyntaxKind(284);
    pub const LastJSDocTagNode: SyntaxKind = SyntaxKind(294);
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
    fn from(v: NewLineKind) -> Self {
        match v {
            NewLineKind::CarriageReturnLineFeed => 0,
            NewLineKind::LineFeed => 1,
            _ => -1,
        }
    }
}

pub struct ScriptKind(i32);
impl ScriptKind {
    pub const Unknown: i32 = 0;
    pub const JS: i32 = 1;
    pub const JSX: i32 = 2;
    pub const TS: i32 = 3;
    pub const TSX: i32 = 4;
    pub const External: i32 = 5;
}
pub struct ScriptTarget(i32);
impl ScriptTarget {
    pub const ES3: i32 = 0;
    pub const ES5: i32 = 1;
    pub const ES2015: i32 = 2;
    pub const ES2016: i32 = 3;
    pub const ES2017: i32 = 4;
    pub const ESNext: i32 = 5;
    pub const Latest: i32 = 5;
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

pub struct EmitHint(i32);
impl EmitHint {
    pub const SourceFile: i32 = 0;
    pub const Expression: i32 = 1;
    pub const IdentifierName: i32 = 2;
    pub const Unspecified: i32 = 3;
}