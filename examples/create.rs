extern crate chakracore as js;
extern crate typescript_ts;

use std::io::prelude::*;
use std::fs::File;
use js::Property;

fn main() {
    let runtime = js::Runtime::new().unwrap();
    let context = js::Context::new(&runtime).unwrap();
    let guard = context.make_current().unwrap();

    let js = typescript_ts::Js::new(&guard);
    let ts = js.ts();

    println!("version: {}", ts.version());
    
    // let sf = ts.createSourceFile("someFileName.ts", "", ts::ScriptTargetConst::Latest, /*setParentNodes*/ false, ts::ScriptKind::TS);
    // let sf = ts.createSourceFile("someFileName.ts", "", typescript_ts::ts::ScriptTarget::ESNext, /*setParentNodes*/ Some(false), Some(typescript_ts::ts::ScriptKind::TS));

    // let p = ts.createPrinter();
    let n = ts.createNode(3, -1, -1);
}
