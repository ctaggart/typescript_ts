extern crate chakracore as js;

use std::io::prelude::*;
use std::fs::File;

// struct Js<'a> {
pub struct Js<'a> {
    // runtime: js::Runtime,
    // context: js::Context,
    // guard: &'a js::context::ContextGuard<'a>,
    guard: &'a js::context::ContextGuard<'a>,
}

// impl<'a> Js<'a> {
impl<'a> Js<'a> {
    
    // fn new(guard: &js::context::ContextGuard) -> Js<'a> {
    pub fn new(guard: &'a js::context::ContextGuard<'a>) -> Js<'a> {
        // let runtime = js::Runtime::new().unwrap();
        // let context = js::Context::new(&runtime).unwrap();
        // let guard = context.make_current().unwrap();

        let js = r"C:\Users\camer\ts\TsAst\node_modules\typescript\lib\typescript.js";
        let mut file = File::open(js).expect("unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("unable to read the file");

        js::script::eval(&guard, &contents).expect("invalid JavaScript code");

        // Js { runtime: runtime, context: context, guard: guard }
        Js { guard: guard }
    }

    pub fn version(&self) -> String {
        let ts = self.guard.global().get(self.guard, &js::Property::new(self.guard, "ts")).into_object().unwrap();
        ts.get(self.guard, &js::Property::new(self.guard, "version")).to_string(self.guard)
    }

}
