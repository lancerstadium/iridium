pub mod vm;
pub mod instruction;
pub mod repl;
pub mod item;


fn main() {
    let mut repl_demo = crate::repl::REPL::new();
    repl_demo.run();
}
