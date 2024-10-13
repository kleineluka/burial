// imports
use swc_common::{sync::Lrc, FileName, SourceMap, Spanned};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, EsSyntax};
use swc_ecma_ast::{Decl, Module, ModuleItem, Pat, Program, Stmt};

// we will need this multiple times, so pack it into a function
pub struct ObfuscationData {
    pub shufflers: Vec<String>,
    pub shuffler_data: String,
    pub full_ast : Module,
    pub game_ast : Program,
}

// extract what the game is using to shuffle the data around
pub fn get_shufflers(ast: &Module) -> Vec<String> {
    // tcoaal stores the shufflers in the 3rd and 4th items in the body
    let mut shufflers = Vec::new();
    let item = &ast.body[2];
    let item2 = &ast.body[3];
    // we want the first declaration in the variable declaration
    let decl = match item {
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(boxed_var_decl))) => &boxed_var_decl.decls,
        _ => panic!("Expected a variable declaration"),
    };
    let decl2 = match item2 {
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(boxed_var_decl))) => &boxed_var_decl.decls,
        _ => panic!("Expected a variable declaration"),
    };
    // get the name of the variable
    let name = match &decl[0].name {
        Pat::Ident(ident) => ident.sym.to_string(),
        _ => panic!("Expected an identifier"),
    };
    let name2 = match &decl2[0].name {
        Pat::Ident(ident) => ident.sym.to_string(),
        _ => panic!("Expected an identifier"),
    };
    // push and return
    shufflers.push(name);
    shufflers.push(name2);
    shufflers

}

// get relevant information from the obfuscated source code
pub fn parse_js(source: &str) -> ObfuscationData {
    // start by reading the full obfuscated javascript
    let cm: Lrc<SourceMap> = Default::default();
    let source_file = cm.new_source_file(Lrc::new(FileName::Custom("ful,.js".into())), source.into());
    let lexer = Lexer::new(
        Syntax::Es(EsSyntax::default()),
        Default::default(),
        StringInput::from(&*source_file),
        None,
    );
    // parse it into an AST
    let mut parser = Parser::new_from(lexer);
    let ast: Module = parser
        .parse_module()
        .expect("Failed to parse JavaScript code");
    // get the shufflers from the AST
    let shufflers = get_shufflers(&ast);
    let mut shuffler_data = &source[..ast.body[3].span().hi.0 as usize];
    shuffler_data = &shuffler_data[..shuffler_data.len() - 1];
    // find where the specific game code is
    let relevant_node: &ModuleItem = &ast.body[3];
    let end_index: usize = relevant_node.span().hi.0 as usize; // Convert Span to usize
    // get the game code (for some reason it parses it without the first `)
    let source_data = format!("'{}", &source[end_index..] );
    let source_file_new = cm.new_source_file(
        Lrc::new(FileName::Custom("game.js".into())),
        source_data.into(),
    );
    let lexer_new = Lexer::new(
        Syntax::Es(EsSyntax::default()),
        Default::default(),
        StringInput::from(&*source_file_new),
        None,
    );
    // parse the game code into an AST
    let mut parser_new = Parser::new_from(lexer_new);
    let source_ast: Program = parser_new
        .parse_program()
        .expect("Failed to parse new JavaScript code");
    // store all relevant data in a struct
    ObfuscationData {
        shufflers,
        shuffler_data: shuffler_data.to_string(),
        full_ast: ast,
        game_ast: source_ast,
    }
}

pub fn clean_code(source: &str, transformers: &str) {
    // start by parsing the JS code (again..)
    let obfuscation_data = parse_js(source);
}