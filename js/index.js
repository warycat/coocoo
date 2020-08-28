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

document.body.classList.add("loading");

var inputElement;
var input_dict = {};

document.getElementById('file_upload').addEventListener('change', function (e) {
    inputElement = event.target.files;
    for (var i = 0; i < inputElement.length; i++) {
        if (input_dict[inputElement[i].name] == undefined) {
            var image_object = document.createElement("option");
            var image_object_name = document.createTextNode(inputElement[i].name);
            image_object.appendChild(image_object_name);
            document.getElementById("file_list").appendChild(image_object);
        }

        var img = document.createElement("img");
        img.src = URL.createObjectURL(inputElement[i]);
        input_dict[inputElement[i].name] = img;
    }

    console.log(input_dict)
}, false);


document.getElementById('file_delete').onclick = function () {
    // var selected = $(file_upload).find("option:selected");
    // console.log(selected);
    console.log(input_dict)
    // selected.remove();
    // var selected_name = selected.text();
    // input_dict[selected_name].remove();
};

import("../pkg/index.js").catch(console.error).then((wasm) => {
    document.getElementById('run').onclick = function () {
        this.disabled = true;
        // get coocoo code from input
        var code_in = document.getElementById("code_in").value;

        // code2wasm to compile coocoo into wasm
        var buffer = wasm.code2wasm(code_in);
        // turn wasm into wat using wabt 
        var wasm_mod = new WebAssembly.Module(buffer);
        var module = wabt.readWasm(buffer, { readDebugNames: true });
        module.generateNames();
        module.applyNames();
        var wast = module.toText({
            foldExprs: true,
            inlineExport: false
        });
        // output wat to webpage
        document.getElementById("code_out").value = wast;
        const wasmInstance = new WebAssembly.Instance(wasm_mod, features);
        const { main } = wasmInstance.exports;
        document.getElementById("code_result").value = main();
        console.log(main())

    };

    document.getElementById('download_button').onclick = function () {
        this.href = document.getElementById("imageCanvas_modified").toDataURL();
        this.download = "image.png";
    };
});

