extern crate chakracore as js;
extern crate typescript_ts;

use std::io::prelude::*;
use std::fs::File;
use js::Property;

fn main() {
    let runtime = js::Runtime::new().unwrap();
    let context = js::Context::new(&runtime).unwrap();
    let guard = context.make_current().unwrap();

    let ts = typescript_ts::Js::new(&guard);

    println!("version: {}", ts.version());
}
