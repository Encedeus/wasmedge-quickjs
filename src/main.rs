#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;
use include_dir::{include_dir, Dir};

// fn args_parse() -> (String, Vec<String>) {
//     use argparse::ArgumentParser;
//     let mut file_path = String::new();
//     let mut res_args: Vec<String> = vec![];
//     {
//         let mut ap = ArgumentParser::new();
//         ap.refer(&mut file_path)
//             .add_argument("file", argparse::Store, "js file")
//             .required();
//         ap.refer(&mut res_args)
//             .add_argument("arg", argparse::List, "arg");
//         ap.parse_args_or_exit();
//     }
//     (file_path, res_args)
// }
//

fn main() {
    use wasmedge_quickjs as q;
    static JS_SRC: Dir = include_dir!("../test/js");

    let mut rt = q::Runtime::new();  
    rt.run_with_context(|ctx| {
        // let mut rest_arg = args_parse();
        let file_path = String::from(JS_SRC.get_file("main.js").unwrap().path().to_str().unwrap());
        
        let code = JS_SRC.get_file("main.js").unwrap().contents_utf8().unwrap().to_string();

        ctx.put_args(vec![file_path.clone()]);
        ctx.eval_module_str(code, &file_path);
        // match code {
        //     Ok(code) => {
        //         // rest_arg.insert(0, file_path.clone());
        //         // ctx.put_args(rest_arg);
        //         ctx.eval_module_str(code, &file_path);
        //     }
        //     Err(e) => {
        //         eprintln!("{}", e.to_string());
        //     }
        // }
        ctx.js_loop().unwrap();
    });
}
