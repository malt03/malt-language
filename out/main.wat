(module
    (import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
    (func $_start
        call $_exit)
    (memory 0)
    (export "memory" (memory 0))
    (export "_start" (func $_start))
)