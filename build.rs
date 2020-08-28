extern crate lalrpop;

fn main() {
    // process_root processes src directory
    // and converting lalrpop files into rs files
    // returns io:;Result<()>
    lalrpop::process_root().unwrap();
}
