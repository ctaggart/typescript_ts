#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
extern crate chakracore;

use std::io::prelude::*;
use std::fs::File;

pub struct Js<'a> {
    guard: &'a chakracore::context::ContextGuard<'a>,
}

impl<'a> Js<'a> {
    pub fn new(guard: &'a chakracore::context::ContextGuard<'a>) -> Js<'a> {

        let js = r"C:\Users\camer\ts\TsAst\node_modules\typescript\lib\typescript.js";
        let mut file = File::open(js).expect("unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("unable to read the file");

        chakracore::script::eval(&guard, &contents).expect("invalid JavaScript code");

        Js { guard: guard }
    }

    pub fn ts(&self) -> ts::Ts<'a> {
        // let object = self.guard.global().get(self.guard, &chakracore::Property::new(self.guard, "ts")).into_object().unwrap();
        // ts::Ts { guard: self.guard, object: object }
        ts::Ts::new(self.guard)
    }
}

pub mod ts {

    extern crate chakracore;

    // declare namespace ts
    pub struct Ts<'a> {
        guard: &'a chakracore::context::ContextGuard<'a>,
        object: chakracore::value::Object,
    }

    impl<'a> Ts<'a> {
        pub fn new(guard: &'a chakracore::context::ContextGuard<'a>) -> Ts<'a> {
            let object = guard.global().get(guard, &chakracore::Property::new(guard, "ts")).into_object().unwrap();
            Ts { guard: guard, object: object }
        }

        pub fn version(&self) -> String {
            self.object.get(self.guard, &chakracore::Property::new(self.guard, "version")).to_string(self.guard)
        }

        pub fn createSourceFile(&self, fileName: &str, sourceText: &str, languageVersion: ScriptTarget, setParentNodes: Option<bool>, scriptKind: Option<ScriptKind>) -> SourceFileImpl<'a> {
            println!("do createSourceFile");
            let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createSourceFile")).into_function().unwrap();
            println!("got function createSourceFile");

            // println!("file");
            // let file = &chakracore::value::String::new(self.guard, fileName).into();
            // println!("st");
            // let st = &chakracore::value::String::new(self.guard, sourceText).into();
            // println!("lv");
            // let lv = &chakracore::value::Number::new(self.guard, 5).into();
            // println!("f");
            // let f = &chakracore::value::Boolean::new(self.guard, false).into();
            // println!("sk");
            // let sk = &chakracore::value::Number::new(self.guard, 3).into();
            // println!("all params");

            // println!("object: {:?}", self.object.);

            // let rv = function.call_with_this(self.guard, &self.object, &[
            //     file,
            //     // st,
            //     // //  &chakracore::value::Number::new(self.guard, languageVersion).into(), TODO
            //     // lv,
            //     // f, // TODO
            //     // // &chakracore::value::Number::new(self.guard, ScriptKind::TS).into(), // TODO
            //     // sk,
            // ]);
            let rv = function.call_with_this(self.guard, &self.object, &[]);
            println!("rv: {:?}", rv);
            
            let object = rv.unwrap().into_object().unwrap();
            SourceFileImpl { guard: self.guard, object: object }
        }

        pub fn createPrinter(&self) -> SourceFileImpl<'a> { // TODO Printer
            println!("do createPrinter");
            let function = self.object.get(self.guard, &chakracore::Property::new(self.guard, "createPrinter")).into_function().unwrap();
            println!("got function createPrinter");

            let rv = function.call_with_this(self.guard, &self.object, &[]);
            println!("rv: {:?}", rv);
            
            let object = rv.unwrap().into_object().unwrap();
            SourceFileImpl { guard: self.guard, object: object } // TODO Printer
        }
    }


    // interface TextRange {
    //     pos: number;
    //     end: number;
    // }
    trait TextRange {}

    // interface Node extends TextRange {
    //     kind: SyntaxKind;
    //     flags: NodeFlags;
    //     decorators?: NodeArray<Decorator>;
    //     modifiers?: ModifiersArray;
    //     parent?: Node;
    // }
    trait Node: TextRange {}

    // interface Declaration extends Node {
    //     _declarationBrand: any;
    // }
    trait Declaration: Node {}

    // interface SourceFile extends Declaration {
    //     kind: SyntaxKind.SourceFile;
    //     statements: NodeArray<Statement>;
    //     endOfFileToken: Token<SyntaxKind.EndOfFileToken>;
    //     fileName: string;
    //     text: string;
    //     amdDependencies: AmdDependency[];
    //     moduleName: string;
    //     referencedFiles: FileReference[];
    //     typeReferenceDirectives: FileReference[];
    //     languageVariant: LanguageVariant;
    //     isDeclarationFile: boolean;
    //     /**
    //      * lib.d.ts should have a reference comment like
    //      *
    //      *  /// <reference no-default-lib="true"/>
    //      *
    //      * If any other file has this comment, it signals not to include lib.d.ts
    //      * because this containing file is intended to act as a default library.
    //      */
    //     hasNoDefaultLib: boolean;
    //     languageVersion: ScriptTarget;
    // }
    // interface SourceFile {
    //     getLineAndCharacterOfPosition(pos: number): LineAndCharacter;
    //     getLineEndOfPosition(pos: number): number;
    //     getLineStarts(): number[];
    //     getPositionOfLineAndCharacter(line: number, character: number): number;
    //     update(newText: string, textChangeRange: TextChangeRange): SourceFile;
    // }
    trait SourceFile: Declaration {
    }

    
    pub struct SourceFileImpl<'a> {
        guard: &'a chakracore::context::ContextGuard<'a>,
        object: chakracore::value::Object,
    }

    impl<'a> TextRange for SourceFileImpl<'a> {
    }
    impl<'a> Node for SourceFileImpl<'a> {
    }
    impl<'a> Declaration for SourceFileImpl<'a> {
    }
    impl<'a> SourceFile for SourceFileImpl<'a> {
    }



    // print

        // function createSourceFile(fileName: string, sourceText: string, languageVersion: ScriptTarget, setParentNodes?: boolean, scriptKind?: ScriptKind): SourceFile;

    // const sf = ts.createSourceFile(filename, source, ts.ScriptTarget.Latest);
    // ts.forEachChild(root, node => {

    // ts.SyntaxKind) {
    //     return kind === ts.SyntaxKind.ModuleDeclaration



    // create

    //     return ts.createIdentifier("abc");
    // }

    // const resultFile = ts.createSourceFile("someFileName.ts", "", ts.ScriptTarget.Latest, /*setParentNodes*/ false, ts.ScriptKind.TS);
    // const printer = ts.createPrinter({
    //     newLine: ts.NewLineKind.LineFeed,
    // });
    // const result = printer.printNode(ts.EmitHint.Unspecified, makeFactorialFunction(), resultFile);

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
        enumKeyword = 83,
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
        enumDeclaration = 232,
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
        enumMember = 264,
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
    pub trait SystaxKindConst {
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
    pub enum ModifierFlags {
        None = 0,
        Export = 1,
        Ambient = 2,
        Public = 4,
        Private = 8,
        Protected = 16,
        Static = 32,
        Readonly = 64,
        Abstract = 128,
        Async = 256,
        Default = 512,
        Const = 2048,
        HasComputedFlags = 536870912,
        AccessibilityModifier = 28,
        ParameterPropertyModifier = 92,
        NonPublicAccessibilityModifier = 24,
        TypeScriptModifier = 2270,
        ExportDefault = 513,
    }
    pub enum JsxFlags {
        None = 0,
        /** An element from a named property of the JSX.IntrinsicElements interface */
        IntrinsicNamedElement = 1,
        /** An element inferred from the string index signature of the JSX.IntrinsicElements interface */
        IntrinsicIndexedElement = 2,
        IntrinsicElement = 3,
    }

    pub enum ModuleKind {
        None = 0,
        CommonJS = 1,
        AMD = 2,
        UMD = 3,
        System = 4,
        ES2015 = 5,
        ESNext = 6,
    }
    pub enum JsxEmit {
        None = 0,
        Preserve = 1,
        React = 2,
        ReactNative = 3,
    }
    pub enum NewLineKind {
        CarriageReturnLineFeed = 0,
        LineFeed = 1,
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
    pub enum LanguageVariant {
        Standard = 0,
        JSX = 1,
    }
    pub enum WatchDirectoryFlags {
        None = 0,
        Recursive = 1,
    }

    pub enum Extension {
        Ts = 0,
        Tsx = 1,
        Dts = 2,
        Js = 3,
        Jsx = 4,
        // LastTypeScriptExtension = 2,
    }

    pub enum EmitFlags {
        SingleLine = 1,
        AdviseOnEmitNode = 2,
        NoSubstitution = 4,
        CapturesThis = 8,
        NoLeadingSourceMap = 16,
        NoTrailingSourceMap = 32,
        NoSourceMap = 48,
        NoNestedSourceMaps = 64,
        NoTokenLeadingSourceMaps = 128,
        NoTokenTrailingSourceMaps = 256,
        NoTokenSourceMaps = 384,
        NoLeadingComments = 512,
        NoTrailingComments = 1024,
        NoComments = 1536,
        NoNestedComments = 2048,
        HelperName = 4096,
        ExportName = 8192,
        LocalName = 16384,
        InternalName = 32768,
        Indented = 65536,
        NoIndentation = 131072,
        AsyncFunctionBody = 262144,
        ReuseTempVariableScope = 524288,
        CustomPrologue = 1048576,
        NoHoisting = 2097152,
        HasEndOfDeclarationMarker = 4194304,
        Iterator = 8388608,
        NoAsciiEscaping = 16777216,
    }

    pub enum EmitHint {
        SourceFile = 0,
        Expression = 1,
        IdentifierName = 2,
        Unspecified = 3,
    }
}