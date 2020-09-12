mod ast;
use ast::Compile;
use std::collections::HashMap;
use walrus::FunctionId;
use walrus::*;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub coocoo);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

struct Compiler {
    module: Module,
    src: String,
    function_ids: HashMap<String, FunctionId>,
}

impl Compiler {
    fn new(src: String) -> Self {
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        let function_ids = HashMap::new();
        Compiler {
            module,
            src,
            function_ids,
        }
    }

    fn import_library_module(&mut self, wasm_buffer: Vec<u8>) {
        let lib = match Module::from_buffer(&wasm_buffer) {
            Ok(module) => module,
            Err(_) => {
                log("Module import error");
                return;
            }
        };

        for func in lib.funcs.iter() {
            let func_name = func.name.as_ref().unwrap().to_string();
            self.function_ids.insert(func_name, func.id());
            let f_n = String::from(func.name.as_ref().unwrap().to_string());
            log(&f_n);
        }
    }

    fn compile(&mut self, wasm_buffer: Vec<u8>) -> Vec<u8> {
        self.function_ids.clear();
        self.import_library_module(wasm_buffer);
        let functions = coocoo::ProgramParser::new().parse(&self.src).unwrap();
        for function in functions {
            let s = format!("{:?}", function);
            let mut params: Vec<ValType> = vec![];
            let mut args: Vec<LocalId> = vec![];
            let mut local_ids: HashMap<String, LocalId> = HashMap::new();
            for param in &function.prototype.params {
                params.push(ValType::F64);
                let id = self.module.locals.add(ValType::F64);
                local_ids.insert(param.to_string(), id);
                args.push(id);
            }
            let mut function_builder =
                FunctionBuilder::new(&mut self.module.types, &params, &[ValType::F64]);
            let mut builder: InstrSeqBuilder = function_builder.func_body();
            function.compile(&mut builder, &local_ids, &self.function_ids);
            let function_id = function_builder.finish(args, &mut self.module.funcs);
            self.function_ids
                .insert(function.prototype.name.to_string(), function_id);
            self.module
                .exports
                .add(&function.prototype.name, function_id);
            log(&s);
        }
        self.module.emit_wasm()
    }
}

#[wasm_bindgen]
pub fn code2wasm(src: String, wasm_buffer: Vec<u8>) -> Vec<u8> {
    let mut compiler = Compiler::new(src);
    compiler.compile(wasm_buffer)
}

// #[wasm_bindgen]
// pub fn get_image_name(code: String) -> String {
//     let image_load = code.split_whitespace().next().unwrap();
//     let image_name = &image_load[11..image_load.chars().count() - 1];
//     image_name.to_owned()
// }

// #[wasm_bindgen]
// pub fn process(image_data: ImageData, code: String) -> ImageData {
//     // get image data
//     let w = image_data.width();
//     let h = image_data.height();
//     let data: Clamped<Vec<u8>> = image_data.data();

//     // get photon image object
//     let mut pimage = PhotonImage::new(data.to_vec(), w, h);
//     // handle statements
//     for statement in code.split_whitespace() {
//         match statement {
//             "grayscale" => photon_rs::monochrome::grayscale(&mut pimage),
//             "blur" => photon_rs::conv::gaussian_blur(&mut pimage, 3),
//             _ => {}
//         }
//     }
//     // return image data
//     pimage.get_image_data()
// }
