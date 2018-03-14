extern crate typescript_ts;

fn main() {
    let (_runtime, context) = typescript_ts::new_context();
    let guard = typescript_ts::new_guard(&context);
    
    // let js = typescript_ts::read_js();
    // typescript_ts::eval_js(&guard, &js);

    // let ts = typescript_ts::ts(&guard);


    // https://github.com/darfink/chakracore-rs/issues/21
    let js = typescript_ts::Js::new(&guard);
    let ts = js.ts();

    println!("version: {}", ts.version());
}