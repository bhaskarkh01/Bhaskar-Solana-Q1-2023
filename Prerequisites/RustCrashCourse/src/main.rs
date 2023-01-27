mod arrays;
mod cli;
mod conditional;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    arrays::run();
    cli::run();
    conditional::run();
    enums::run();
    functions::run();
    loops::run();
    pointer_ref::run();
    structs::run();
    tuples::run();
    vectors::run();
}
