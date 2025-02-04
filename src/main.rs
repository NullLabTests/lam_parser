mod prolog_parser;
use prolog_parser::{parse_program, compile_clause, Instruction};

fn main() {
    let program = r#"
        parent(john, mary).
        parent(mary, susan).
        ancestor(X, Y) :- parent(X, Y).
        ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).
    "#;

    match parse_program(program) {
        Ok((_rest, clauses)) => {
            println!("Parsed Prolog Clauses:");
            for clause in &clauses {
                println!("{:?}", clause);
            }
            println!("\nCompiled LAM Instructions:");
            for clause in clauses {
                let instructions = compile_clause(clause);
                for instr in instructions {
                    println!("{:?}", instr);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing program: {:?}", e);
        }
    }
}
