(module
    (import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
    (func $_start
        i32.const 2
        i32.const 3
        i32.const 5
        i32.const 1
        i32.sub
        i32.const 4
        i32.add
        i32.mul
        i32.const 2
        i32.div_s
        i32.add
        call $_exit)
    (memory 0)
    (export "memory" (memory 0))
    (export "_start" (func $_start))
)