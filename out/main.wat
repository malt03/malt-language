(module
(import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
(func $main (result i32)(local $foo i32)(local $bar i32)
(local.set $foo(i32.add(i32.const 2)(i32.div_s(i32.mul(i32.const 3)(i32.add(i32.sub(i32.const 5)(i32.const 1))(i32.const 1)))(i32.const 3))))
(local.set $bar(i32.sub(i32.const 10)(i32.const 4)))
(i32.add(local.get $foo)(local.get $bar)))
(func $baz (result i32)
(i32.const 10))
(func $_start
call $main
call $_exit)
(memory 0)
(export "memory" (memory 0))
(export "_start" (func $_start)))