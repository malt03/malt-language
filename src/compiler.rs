mod tokens;
mod syntax_tree;

use std::io;

use tokens::PeekableTokens;
use syntax_tree::{SyntaxTree, llvm_generator};
use inkwell::support::LLVMString;
use inkwell::{OptimizationLevel, targets};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::values::FloatValue;
use std::path::Path;

#[derive(Debug)]
pub enum Error<'a> {
    IO(io::Error),
    SyntaxTree(syntax_tree::Error<'a>),
    LLVMString(LLVMString),
    LLVMGenerator(llvm_generator::Error<'a>)
}
impl<'a> From<io::Error> for Error<'a> {
    fn from(err: io::Error) -> Self { Error::IO(err) }
}
impl<'a> From<syntax_tree::Error<'a>> for Error<'a> {
    fn from(err: syntax_tree::Error<'a>) -> Self { Error::SyntaxTree(err) }
}
impl<'a> From<LLVMString> for Error<'a> {
    fn from(err: LLVMString) -> Self { Error::LLVMString(err) }
}
impl<'a> From<llvm_generator::Error<'a>> for Error<'a> {
    fn from(err: llvm_generator::Error<'a>) -> Self { Error::LLVMGenerator(err) }
}
impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => err.fmt(f),
            Error::SyntaxTree(err) => err.fmt(f),
            Error::LLVMString(err) => err.fmt(f),
            Error::LLVMGenerator(err) => err.fmt(f),
        }
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

pub fn compile<'a>(text: &'a str) -> Result<'a, ()> {
    let tokens = PeekableTokens::new(&text);
    let syntax_tree = SyntaxTree::new(tokens)?;

    let context = Context::create();
    let llvm = llvm_generator::LLVMGenerator::new(&context);
    let main_function = llvm.function(&syntax_tree.root)?;

    Target::initialize_all(&Default::default());
    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple)?;
    let target_machine = target.create_target_machine(&triple, cpu.to_str().unwrap(), features.to_str().unwrap(), OptimizationLevel::Default, RelocMode::PIC, CodeModel::Default).unwrap();
    target_machine.write_to_file(&llvm.module, FileType::Object, Path::new("./out.o"))?;
    std::process::Command::new("gcc")
        .args(vec!["out.o".into(), "-o".into(), "a.out"])
        .output()?;

    // unsafe {
    //     llvm.module.create_execution_engine()?.run_function(main_function, &[]);
    // }
    
    Ok(())
}

// pub fn compile<'a, W: io::Write>(text: &'a str, mut writer: W) -> Result<'a, ()> {
//     writer.write_all(br#"(module
// (import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
// "#)?;

//     let tokens = PeekableTokens::new(&text);
//     let syntax_tree = SyntaxTree::new(tokens)?;
//     syntax_tree.write_wasm(&mut writer)?;

//     writer.write_all(br#"
// (func $_start
// call $main
// call $_exit)
// (memory 0)
// (export "memory" (memory 0))
// (export "_start" (func $_start)))"#)?;
//     writer.flush()?;

//     Ok(())
// }
