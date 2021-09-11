(module
    (import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
    (func $_start
        i32.const 2
        i32.const 21
        i32.add
        i32.const 1
        i32.sub
        call $_exit)
    (memory 0)
    (export "memory" (memory 0))
    (export "_start" (func $_start))
)