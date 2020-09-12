# [walrus_demo](https://warycat.github.io/walrus_demo/)

This demo uses [lalrpop](https://github.com/lalrpop/lalrpop) (frontend) and [walrus](https://github.com/rustwasm/walrus) (backend) to compile [kaleidoscope](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html) languange into wasm. The compiler itself is also a wasm module.

```
def add(a, b) {
    a + b
}

def sub(a, b) {
    a - b
}

def main() {
    add(2, add(4,8)) + sub(2,4)
}

```

```
(module
  (type $t0 (func (result f64)))
  (type $t1 (func (param f64 f64) (result f64)))
  (func $main (type $t0) (result f64)
    (f64.add
      (call $add
        (f64.const 0x1p+1 (;=2;))
        (call $add
          (f64.const 0x1p+2 (;=4;))
          (f64.const 0x1p+3 (;=8;))))
      (call $sub
        (f64.const 0x1p+1 (;=2;))
        (f64.const 0x1p+2 (;=4;)))))
  (func $add (type $t1) (param $p0 f64) (param $p1 f64) (result f64)
    (f64.add
      (local.get $p0)
      (local.get $p1)))
  (func $sub (type $t1) (param $p0 f64) (param $p1 f64) (result f64)
    (f64.sub
      (local.get $p0)
      (local.get $p1)))
  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "main" (func $main)))


```