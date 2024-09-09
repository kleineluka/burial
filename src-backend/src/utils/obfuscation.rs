/*
// imports
use oxc_ast::ast::{AstNode, Program, Statement, VariableDeclaration, Identifier};
use oxc_parser::Parser;
use oxc_allocator::Allocator;
use oxc_diagnostics::Error;

// take javascript source code and return an abstract syntax tree
fn parse_javascript(source: &str) -> Result<AstNode, Vec<Error>> {
    // initialize + parse into ast
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, source);
    let ast = parser.parse();
    // check for errors
    if !ast.errors.is_empty() {
        return Err(ast.errors);
    }
    // return the parsed AST
    Ok(ast.program)
}

// to-do
fn enforce_shape(ast: &AstNode) {
    // to-do
}

// extract the names of the shufflers from the AST
fn get_shufflers(source: &str, program: &Program) -> (Vec<String>, &str) {
    // extract shufflers from ast.body[2] and ast.body[3]
    let mut shufflers = Vec::new();
    let mut end_pos = 0;
    for i in 2..=3 {
        if let Some(Statement::VariableDeclaration(decl)) = program.body.get(i) {
            if let Some(VariableDeclaration::Identifier(id)) = &decl.declarations.get(0).unwrap().id {
                shufflers.push(id.name.clone());
            }
            if i == 3 {
                // store the end position for slicing the source (ast.body[3].end)
                end_pos = decl.span.end;
            }
        }
    }
    // slice the source up to the end position of ast.body[3]
    let shuffler_data = &source[..end_pos as usize];
    (shufflers, shuffler_data)
}

// wip reimplemntation of grimoire by Basil.cafe
pub fn deobfuscate(source: &str) -> Result<String, Vec<Error>> {
    // parse into ast + extract shufflers
    let ast = parse_javascript(source)?;
    let (shufflers, shuffler_data) = get_shufflers(source, &ast);
    // parse the program after slicing
    let start_pos = program.body.get(3).unwrap().span().end;
    let source_data = &source[start_pos as usize..];
    let allocator = Allocator::default();
    let new_ast = Parser::new(&allocator, source_data).parse_program().unwrap();
    Ok("".to_string())
}
*/