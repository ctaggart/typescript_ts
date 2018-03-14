extern crate typescript_ts;

fn main() {
    let (_runtime, context) = typescript_ts::new_context();
    let guard = typescript_ts::new_guard(&context);
    let js = typescript_ts::Js::new(&guard);
    let ts = js.ts();
    println!("version: {}", ts.version());
}