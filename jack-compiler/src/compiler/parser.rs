use crate::compiler::tokenizer::*;
use std::fmt::Debug;
use std::iter::Peekable;
use std::slice::Iter;

type Tokens<'a> = Peekable<Iter<'a, Token>>;

pub fn parse<S>(input: S) -> Class
where
    S: AsRef<str>,
{
    let tokens = tokenize(input);
    let mut token_iterator: Tokens = tokens.iter().peekable();

    parse_class(&mut token_iterator)
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: String,
    pub variable_declarations: Vec<ClassVariableDeclaration>,
    pub subroutine_declarations: Vec<SubroutineDeclaration>,
}

#[derive(Debug, PartialEq)]
pub struct ClassVariableDeclaration {
    pub scope: ClassVariableScope,
    pub data_type: DataType,
    pub names: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum ClassVariableScope {
    Static,
    Field,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Char,
    Boolean,
    Class(String),
}

#[derive(Debug, PartialEq)]
pub struct SubroutineDeclaration {
    pub kind: SubroutineKind,
    pub return_type: SubroutineReturnType,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}

#[derive(Debug, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, PartialEq)]
pub enum SubroutineReturnType {
    Void,
    Returning(DataType),
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, PartialEq)]
pub struct SubroutineBody {
    pub variable_declarations: Vec<VariableDeclaration>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub data_type: DataType,
    pub names: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(String, Option<Expression>, Expression),
    If(Expression, Vec<Statement>, Vec<Statement>),
    While(Expression, Vec<Statement>),
    Do(Option<String>, String, Vec<Expression>),
    Return(Option<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub term: Box<Term>,
    pub pairs: Vec<(Operation, Term)>,
}

#[derive(Debug, PartialEq)]
pub enum Term {
    IntegerConstant(u16),
    StringConstant(String),
    Keyword(KeywordConstant),
    Variable(String),
    ArrayAccess(String, Expression),
    Parenthetical(Expression),
    Negate(Box<Term>),
    Not(Box<Term>),
    Call(Option<String>, String, Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    LessThan,
    GreaterThan,
    Equals,
}

#[derive(Debug, PartialEq)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

fn parse_class(tokens: &mut Tokens) -> Class {
    consume(tokens, Token::Keyword("class".into()));
    let name = parse_identifier(tokens);
    consume(tokens, Token::Symbol("{".into()));
    let variable_declarations = parse_class_variable_declarations(tokens);
    let subroutine_declarations = parse_subroutine_declarations(tokens);

    Class {
        name,
        variable_declarations,
        subroutine_declarations,
    }
}

fn parse_class_variable_declarations(
    tokens: &mut Tokens,
) -> Vec<ClassVariableDeclaration> {
    let mut declarations = Vec::new();

    while let Some(Token::Keyword(keyword)) = tokens.peek() {
        match keyword.as_ref() {
            "static" => {
                consume_token(tokens);
                declarations.push(parse_class_variable_declaration(
                    tokens,
                    ClassVariableScope::Static,
                ));
            }
            "field" => {
                consume_token(tokens);
                declarations.push(parse_class_variable_declaration(
                    tokens,
                    ClassVariableScope::Field,
                ));
            }
            _ => break,
        }
    }

    declarations
}

fn parse_class_variable_declaration(
    tokens: &mut Tokens,
    scope: ClassVariableScope,
) -> ClassVariableDeclaration {
    let data_type = parse_data_type(tokens);
    let mut names = vec![parse_identifier(tokens)];

    while let Some(Token::Symbol(symbol)) = tokens.peek() {
        match symbol.as_ref() {
            "," => {
                consume_token(tokens);
                names.push(parse_identifier(tokens));
            }
            _ => break,
        }
    }
    consume(tokens, Token::Symbol(";".into()));

    ClassVariableDeclaration {
        scope,
        data_type,
        names,
    }
}

fn parse_subroutine_declarations(
    tokens: &mut Tokens,
) -> Vec<SubroutineDeclaration> {
    let mut declarations = Vec::new();

    while let Some(Token::Keyword(keyword)) = tokens.peek() {
        match keyword.as_ref() {
            "constructor" => {
                consume_token(tokens);
                declarations.push(parse_subroutine_declaration(
                    tokens,
                    SubroutineKind::Constructor,
                ));
            }
            "function" => {
                consume_token(tokens);
                declarations.push(parse_subroutine_declaration(
                    tokens,
                    SubroutineKind::Function,
                ));
            }
            "method" => {
                consume_token(tokens);
                declarations.push(parse_subroutine_declaration(
                    tokens,
                    SubroutineKind::Method,
                ));
            }
            _ => break,
        }
    }

    declarations
}

fn parse_subroutine_declaration(
    tokens: &mut Tokens,
    kind: SubroutineKind,
) -> SubroutineDeclaration {
    let return_type = parse_return_type(tokens);
    let name = parse_identifier(tokens);
    let parameters = parse_parameters(tokens);
    let body = parse_subroutine_body(tokens);

    SubroutineDeclaration {
        kind,
        return_type,
        name,
        parameters,
        body,
    }
}

fn parse_return_type(tokens: &mut Tokens) -> SubroutineReturnType {
    match tokens.peek() {
        Some(Token::Keyword(keyword)) => match keyword.as_ref() {
            "void" => {
                consume_token(tokens);
                SubroutineReturnType::Void
            }
            _ => panic!("Expected void|type, got {:?}", keyword),
        },
        _ => SubroutineReturnType::Returning(parse_data_type(tokens)),
    }
}

fn parse_parameters(tokens: &mut Tokens) -> Vec<Parameter> {
    let mut parameters = Vec::new();

    consume(tokens, Token::Symbol("(".into()));

    loop {
        match tokens.peek() {
            Some(Token::Symbol(symbol)) => match symbol.as_ref() {
                "," => {
                    consume_token(tokens);
                    parameters.push(parse_parameter(tokens));
                }
                ")" => {
                    consume_token(tokens);
                    break;
                }
                _ => panic!("Expecting ( | , | ) got {}", symbol),
            },
            Some(Token::Keyword(_)) => {
                parameters.push(parse_parameter(tokens));
            }
            token => panic!("Expecting parameter got {:?}", token),
        }
    }

    parameters
}

fn parse_parameter(tokens: &mut Tokens) -> Parameter {
    let data_type = parse_data_type(tokens);
    let name = parse_identifier(tokens);

    Parameter { name, data_type }
}

fn parse_subroutine_body(tokens: &mut Tokens) -> SubroutineBody {
    consume(tokens, Token::Symbol("{".into()));
    let variable_declarations = parse_variable_declarations(tokens);
    let statements = parse_statements(tokens);
    consume(tokens, Token::Symbol("}".into()));

    SubroutineBody {
        variable_declarations,
        statements,
    }
}

fn parse_variable_declarations(
    tokens: &mut Tokens,
) -> Vec<VariableDeclaration> {
    let mut declarations = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::Keyword(keyword)) => match keyword.as_ref() {
                "var" => {
                    consume_token(tokens);
                    declarations.push(parse_variable_declaration(tokens));
                }
                _ => break,
            },
            Some(Token::Symbol(symbol)) => match symbol.as_ref() {
                "," => {
                    consume_token(tokens);
                    declarations.push(parse_variable_declaration(tokens));
                }
                _ => break,
            },
            _ => break,
        }
    }

    declarations
}

fn parse_variable_declaration(tokens: &mut Tokens) -> VariableDeclaration {
    let data_type = parse_data_type(tokens);
    let mut names = vec![parse_identifier(tokens)];

    loop {
        match tokens.peek() {
            Some(Token::Symbol(symbol)) => match symbol.as_ref() {
                "," => {
                    consume_token(tokens);
                    names.push(parse_identifier(tokens));
                }
                ";" => {
                    consume_token(tokens);
                    break;
                }
                _ => panic!("Expected , | ; got {}", symbol),
            },
            token => {
                panic!(
                    "Expected variable declaration identifier, got {:?}",
                    token
                )
            }
        }
    }

    VariableDeclaration { data_type, names }
}

fn parse_statements(tokens: &mut Tokens) -> Vec<Statement> {
    let mut statements = Vec::new();

    while let Some(Token::Keyword(keyword)) = tokens.peek() {
        match keyword.as_ref() {
            "let" => statements.push(parse_let_statement(tokens)),
            "if" => statements.push(parse_if_statement(tokens)),
            "while" => statements.push(parse_while_statement(tokens)),
            "do" => statements.push(parse_do_statement(tokens)),
            "return" => statements.push(parse_return_statement(tokens)),
            _ => break,
        }
    }

    statements
}

fn parse_let_statement(tokens: &mut Tokens) -> Statement {
    consume_token(tokens);
    let name = parse_identifier(tokens);
    let array_index_expression =
        if tokens.peek() == Some(&&Token::Symbol("[".into())) {
            consume_token(tokens);
            let array_index_expression = parse_expression(tokens);
            consume(tokens, Token::Symbol("]".into()));

            Some(array_index_expression)
        } else {
            None
        };
    consume(tokens, Token::Symbol("=".into()));
    let expression = parse_expression(tokens);
    consume(tokens, Token::Symbol(";".into()));

    Statement::Let(name, array_index_expression, expression)
}

fn parse_if_statement(tokens: &mut Tokens) -> Statement {
    consume_token(tokens);

    consume(tokens, Token::Symbol("(".into()));
    let conditional = parse_expression(tokens);
    consume(tokens, Token::Symbol(")".into()));

    consume(tokens, Token::Symbol("{".into()));
    let true_statements = parse_statements(tokens);
    consume(tokens, Token::Symbol("}".into()));

    let false_statements = match tokens.peek() {
        Some(Token::Keyword(keyword)) => match keyword.as_ref() {
            "else" => {
                consume_token(tokens);
                consume(tokens, Token::Symbol("{".into()));
                let statements = parse_statements(tokens);
                consume(tokens, Token::Symbol("}".into()));
                statements
            }
            _ => vec![],
        },
        _ => vec![],
    };

    Statement::If(conditional, true_statements, false_statements)
}

fn parse_while_statement(tokens: &mut Tokens) -> Statement {
    consume_token(tokens);

    consume(tokens, Token::Symbol("(".into()));
    let expression = parse_expression(tokens);
    consume(tokens, Token::Symbol(")".into()));

    consume(tokens, Token::Symbol("{".into()));
    let statements = parse_statements(tokens);
    consume(tokens, Token::Symbol("}".into()));

    Statement::While(expression, statements)
}

fn parse_do_statement(tokens: &mut Tokens) -> Statement {
    consume_token(tokens);
    let identifier = parse_identifier(tokens);

    match parse_call(tokens, identifier) {
        Term::Call(maybe_class, name, expressions) => {
            consume(tokens, Token::Symbol(";".into()));
            Statement::Do(maybe_class, name, expressions)
        }
        _ => panic!("unexpected error"),
    }
}

fn parse_return_statement(tokens: &mut Tokens) -> Statement {
    consume_token(tokens);

    let expression = if tokens.peek() == Some(&&Token::Symbol(";".into())) {
        None
    } else {
        Some(parse_expression(tokens))
    };

    consume(tokens, Token::Symbol(";".into()));

    Statement::Return(expression)
}

fn parse_expression(tokens: &mut Tokens) -> Expression {
    let term = parse_term(tokens);
    let mut pairs = Vec::new();

    while let Some(Token::Symbol(symbol)) = tokens.peek() {
        match symbol.as_ref() {
            "+" | "-" | "*" | "/" | "&" | "|" | "<" | ">" | "=" => {
                let operation = parse_operation(tokens);
                let term = parse_term(tokens);

                pairs.push((operation, term));
            }
            _ => break,
        }
    }

    Expression {
        term: Box::new(term),
        pairs,
    }
}

fn parse_term(tokens: &mut Tokens) -> Term {
    match tokens.next() {
        Some(Token::IntegerConstant(value)) => Term::IntegerConstant(*value),
        Some(Token::StringConstant(value)) => {
            Term::StringConstant(value.clone())
        }
        Some(Token::Keyword(value)) => match value.as_ref() {
            "true" => Term::Keyword(KeywordConstant::True),
            "false" => Term::Keyword(KeywordConstant::False),
            "null" => Term::Keyword(KeywordConstant::Null),
            "this" => Term::Keyword(KeywordConstant::This),
            _ => panic!("Expected true|false|null|this, got {:?}", value),
        },
        Some(Token::Identifier(name)) => match tokens.peek() {
            Some(Token::Symbol(symbol)) => match symbol.as_ref() {
                "[" => {
                    consume_token(tokens);
                    let expression = parse_expression(tokens);
                    consume(tokens, Token::Symbol("]".into()));
                    Term::ArrayAccess(name.clone(), expression)
                }
                "(" | "." => parse_call(tokens, name.clone()),
                _ => Term::Variable(name.clone()),
            },
            _ => Term::Variable(name.clone()),
        },
        Some(Token::Symbol(symbol)) => match symbol.as_ref() {
            "(" => {
                let expression = parse_expression(tokens);
                consume(tokens, Token::Symbol(")".into()));
                Term::Parenthetical(expression)
            }
            "-" => {
                let term = parse_term(tokens);
                Term::Negate(Box::new(term))
            }
            "~" => {
                let term = parse_term(tokens);
                Term::Not(Box::new(term))
            }
            _ => panic!("Expected ( | - | ~, got {:?}", symbol),
        },
        None => panic!("Expected term, got EoF"),
    }
}

fn parse_operation(tokens: &mut Tokens) -> Operation {
    match tokens.next() {
        Some(Token::Symbol(symbol)) => match symbol.as_ref() {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            "&" => Operation::And,
            "|" => Operation::Or,
            "<" => Operation::LessThan,
            ">" => Operation::GreaterThan,
            "=" => Operation::Equals,
            _ => panic!("Expected +|-|*|/|&|||<|>|=, got {:?}", symbol),
        },
        token => panic!("Expected operation, got {:?}", token),
    }
}

fn parse_call(tokens: &mut Tokens, identifier: String) -> Term {
    let (maybe_class, name) = match tokens.next() {
        Some(Token::Symbol(symbol)) => match symbol.as_ref() {
            "(" => (None, identifier),
            "." => {
                let name = parse_identifier(tokens);
                consume(tokens, Token::Symbol("(".into()));
                (Some(identifier), name)
            }
            _ => panic!("Expected ( | ., got {}", symbol),
        },
        token => panic!("Expected ( | ., got {:?}", token),
    };
    let mut expressions = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::Symbol(symbol)) => match symbol.as_ref() {
                ")" => {
                    consume_token(tokens);
                    break;
                }
                "," => {
                    consume_token(tokens);
                }
                _ => expressions.push(parse_expression(tokens)),
            },
            _ => expressions.push(parse_expression(tokens)),
        }
    }

    Term::Call(maybe_class, name, expressions)
}

fn parse_data_type(tokens: &mut Tokens) -> DataType {
    match tokens.next() {
        Some(Token::Keyword(name)) => match name.as_ref() {
            "int" => DataType::Int,
            "char" => DataType::Char,
            "boolean" => DataType::Boolean,
            _ => panic!("Expected data type, got {:?}", name),
        },
        Some(Token::Identifier(name)) => DataType::Class(name.clone()),
        Some(token) => panic!("Expected data type, got {:?}", token),
        None => panic!("Expected data type, got EoF"),
    }
}

fn parse_identifier(tokens: &mut Tokens) -> String {
    match tokens.next() {
        Some(Token::Identifier(name)) => name.clone(),
        Some(token) => panic!("Expected identifier, got {:?}", token),
        None => panic!("Expected identifier, got EoF"),
    }
}

fn consume<T>(tokens: &mut Tokens, expected_token: T)
where
    T: AsRef<Token> + Debug,
{
    if let Some(next_token) = tokens.next() {
        if next_token != expected_token.as_ref() {
            panic!("Expected {:?}, got {:?}", expected_token, next_token);
        }
    } else {
        panic!("Expected {:?}, got EoF", expected_token);
    }
}

fn consume_token(tokens: &mut Tokens) {
    if tokens.next().is_none() {
        panic!("Expected token, got EoF");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse(
                "\
        class Foo {
            static String quux; // end-of-line comment

            function void Bar() {
                while (count < 100) {
                    let count = count + blergh(42, 86);
                }
                do Memory.deAlloc(baz);
                return;
            }
        }
        "
            ),
            Class {
                name: "Foo".into(),
                variable_declarations: vec![ClassVariableDeclaration {
                    scope: ClassVariableScope::Static,
                    data_type: DataType::Class("String".into()),
                    names: vec!["quux".into()]
                }],
                subroutine_declarations: vec![SubroutineDeclaration {
                    kind: SubroutineKind::Function,
                    return_type: SubroutineReturnType::Void,
                    name: "Bar".into(),
                    parameters: vec![],
                    body: SubroutineBody {
                        variable_declarations: vec![],
                        statements: vec![
                            Statement::While(
                                Expression {
                                    term: Box::new(Term::Variable(
                                        "count".into()
                                    )),
                                    pairs: vec![(
                                        Operation::LessThan,
                                        Term::IntegerConstant(100)
                                    )]
                                },
                                vec![Statement::Let(
                                    "count".into(),
                                    None,
                                    Expression {
                                        term: Box::new(Term::Variable(
                                            "count".into()
                                        )),
                                        pairs: vec![(
                                            Operation::Add,
                                            Term::Call(
                                                None,
                                                "blergh".into(),
                                                vec![
                                                Expression {
                                                    term: Box::new(
                                                        Term::IntegerConstant(
                                                            42
                                                        )
                                                    ),
                                                    pairs: vec![]
                                                },
                                                Expression {
                                                    term: Box::new(
                                                        Term::IntegerConstant(
                                                            86
                                                        )
                                                    ),
                                                    pairs: vec![]
                                                }
                                                ]
                                            )
                                        )]
                                    }
                                )]
                            ),
                            Statement::Do(
                                Some("Memory".into()),
                                "deAlloc".into(),
                                vec![Expression {
                                    term: Box::new(Term::Variable(
                                        "baz".into()
                                    )),
                                    pairs: vec![]
                                }]
                            ),
                            Statement::Return(None)
                        ]
                    }
                }]
            }
        );
    }

    #[test]
    fn test_more() {
        let class = parse(
            "\
/** Initializes a new Square Dance game and starts running it. */
class Main {
    static boolean test;    // Added for testing -- there is no static keyword
                            // in the Square files.
    function void main() {
      var SquareGame game;
      let game = SquareGame.new();
      do game.run();
      do game.dispose();
      return;
    }

    function void more() {  // Added to test Jack syntax that is not used in
        var int i, j;       // the Square files.
        var String s;
        var Array a;
        if (false) {
            let s = \"string constant\";
            let s = null;
            let a[1] = a[2];
        }
        else {              // There is no else keyword in the Square files.
            let i = i * (-j);
            let j = j / (-2);   // note: unary negate constant 2
            let i = i | j;
        }
        return;
    }
}
",
        );
        assert_eq!(class.name, "Main");
    }
}
