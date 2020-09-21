var wabt = require("wabt")();

var features = {
    exceptions: false,
    mutable_globals: true,
    sat_float_to_int: false,
    sign_extension: false,
    simd: false,
    threads: false,
    multi_value: false,
    tail_call: false,
    bulk_memory: false,
    reference_types: false,
    console: console
};

var input_files;
var input_images = {};

var exports = {};

const coocooImportObject = {};

window.onload = async function () {
    document.body.classList.add("loading");

    document.getElementById('file_upload').addEventListener('change', function (e) {
        input_files = event.target.files;
        for (var i = 0; i < input_files.length; i++) {
            if (input_images[input_files[i].name] == undefined) {
                var image_option = document.createElement("option");
                var image_option_name = document.createTextNode(input_files[i].name);
                image_option.appendChild(image_option_name);
                document.getElementById("file_list").appendChild(image_option);
            }

            var img = document.createElement("img");
            img.src = URL.createObjectURL(input_files[i]);
            input_images[input_files[i].name] = img;
        }
    }, false);


    document.getElementById('file_delete').onclick = function () {
        var self = document.getElementById("file_list");
        delete input_images[self.value];
        self.remove(self.selectedIndex);
    }

    document.getElementById('file_rename').onclick = function () {
        var self = document.getElementById("file_list");
        var old_name = self.value;
        var new_name = document.getElementById("file_new_name").value;

        if (new_name in input_images) {
            window.alert("please enter another name for this image");
        } else {
            input_images[new_name] = input_images[old_name];
            delete input_images[old_name];

            self.remove(self.selectedIndex);
            var new_option = document.createElement("option");
            var new_option_name = document.createTextNode(new_name);
            new_option.appendChild(new_option_name);
            self.appendChild(new_option);
        }
    };

    document.getElementById('download_button').onclick = function () {
        this.href = document.getElementById("imageCanvas_modified").toDataURL();
        this.download = "image.png";
    };
};


import("../pkg/coocoo.js").then(compiler => {
    function print_wat(buffer) {
        var module = wabt.readWasm(buffer, { readDebugNames: true });
        module.generateNames();
        module.applyNames();
        var wat = module.toText({
            foldExprs: true,
            inlineExport: false
        });

        document.getElementById("code_output").value = wat;
    }

    document.getElementById('run').onclick = async function () {
        var code_input = document.getElementById("code_input").value;

        var response = await fetch("coocoo_library_bg.wasm");
        var library = await response.arrayBuffer();
        var enc = new TextDecoder("utf-8");
        library = enc.decode(library);
        // console.log(typeof library)//string
        output_buffer = compiler.code2wasm(code_input, library);

        var wasm_mod = new WebAssembly.Module(output_buffer);
        const module = await WebAssembly.instantiate(wasm_mod);


        //----------show wat of compiled coocoo code---------
        print_wat(output_buffer);
    }
}).catch(console.error);




