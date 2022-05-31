use crate::compiler::parser::*;
use crate::compiler::tokenizer;

pub fn print_tokens(tokens: Vec<tokenizer::Token>) {
    print_open_tag("tokens", 0);
    for token in tokens {
        use tokenizer::Token::*;
        let (name, value) = match token {
            Keyword(string) => ("keyword", string),
            Symbol(string) => ("symbol", escape(string)),
            IntegerConstant(number) => ("integerConstant", number.to_string()),
            StringConstant(string) => ("stringConstant", string),
            Identifier(string) => ("identifier", string),
        };
        print_element(name, value, 1);
    }
    print_close_tag("tokens", 0);
}

pub fn print_class(class: Class) {
    print_open_tag("class", 0);

    print_element("keyword", "class", 1);
    print_element("identifier", class.name, 1);
    print_element("symbol", "{", 1);

    for declaration in class.variable_declarations {
        print_class_variable_declaration(declaration);
    }

    for declaration in class.subroutine_declarations {
        print_subroutine_declaration(declaration);
    }

    print_element("symbol", "}", 1);

    print_close_tag("class", 0);
}

fn print_class_variable_declaration(
    variable_declaration: ClassVariableDeclaration,
) {
    let scope = match variable_declaration.scope {
        ClassVariableScope::Static => "static",
        ClassVariableScope::Field => "field",
    };

    print_open_tag("classVarDec", 1);
    print_element("keyword", scope, 2);
    print_data_type(variable_declaration.data_type, 2);
    print_identifier_list(variable_declaration.names, 2);
    print_element("symbol", ";", 2);
    print_close_tag("classVarDec", 1);
}

fn print_subroutine_declaration(subroutine_declaration: SubroutineDeclaration) {
    let kind = match subroutine_declaration.kind {
        SubroutineKind::Constructor => "constructor",
        SubroutineKind::Function => "function",
        SubroutineKind::Method => "method",
    };

    print_open_tag("subroutineDec", 1);
    print_element("keyword", kind, 2);
    match subroutine_declaration.return_type {
        SubroutineReturnType::Void => {
            print_element("keyword", "void", 2);
        }
        SubroutineReturnType::Returning(data_type) => {
            print_data_type(data_type, 2);
        }
    };
    print_element("identifier", subroutine_declaration.name, 2);
    print_element("symbol", "(", 2);

    print_open_tag("parameterList", 2);
    for (index, parameter) in
        subroutine_declaration.parameters.into_iter().enumerate()
    {
        if index > 0 {
            print_element("symbol", ",", 3);
        }
        print_parameter(parameter, 3);
    }
    print_close_tag("parameterList", 2);

    print_element("symbol", ")", 2);

    print_subroutine_body(subroutine_declaration.body);

    print_close_tag("subroutineDec", 1);
}

fn print_subroutine_body(body: SubroutineBody) {
    print_open_tag("subroutineBody", 2);
    print_element("symbol", "{", 3);

    for declaration in body.variable_declarations {
        print_variable_declaration(declaration);
    }

    print_open_tag("statements", 3);
    for statement in body.statements {
        print_statement(statement, 4);
    }
    print_close_tag("statements", 3);

    print_element("symbol", "}", 3);
    print_close_tag("subroutineBody", 2);
}

fn print_variable_declaration(declaration: VariableDeclaration) {
    print_open_tag("varDec", 3);
    print_element("keyword", "var", 4);
    print_data_type(declaration.data_type, 4);
    print_identifier_list(declaration.names, 4);
    print_element("symbol", ";", 4);
    print_close_tag("varDec", 3);
}

fn print_statement(statement: Statement, base_indent: usize) {
    match statement {
        Statement::Let(name, maybe_array_index, expression) => {
            print_open_tag("letStatement", base_indent);
            print_element("keyword", "let", base_indent + 1);
            print_element("identifier", name, base_indent + 1);

            match maybe_array_index {
                Some(index_expression) => {
                    print_element("symbol", "[", base_indent + 1);
                    print_expression(index_expression, base_indent + 1);
                    print_element("symbol", "]", base_indent + 1);
                }
                None => {}
            }

            print_element("symbol", "=", base_indent + 1);
            print_expression(expression, base_indent + 1);

            print_element("symbol", ";", base_indent + 1);
            print_close_tag("letStatement", base_indent);
        }
        Statement::If(conditional, true_statements, false_statements) => {
            print_open_tag("ifStatement", base_indent);
            print_element("keyword", "if", base_indent + 1);

            print_element("symbol", "(", base_indent + 1);
            print_expression(conditional, base_indent + 1);
            print_element("symbol", ")", base_indent + 1);

            print_element("symbol", "{", base_indent + 1);
            print_open_tag("statements", base_indent + 1);
            for statement in true_statements {
                print_statement(statement, base_indent + 2);
            }
            print_close_tag("statements", base_indent + 1);
            print_element("symbol", "}", base_indent + 1);

            if !false_statements.is_empty() {
                print_element("keyword", "else", base_indent + 1);
                print_element("symbol", "{", base_indent + 1);
                print_open_tag("statements", base_indent + 1);
                for statement in false_statements {
                    print_statement(statement, base_indent + 2);
                }
                print_close_tag("statements", base_indent + 1);
                print_element("symbol", "}", base_indent + 1);
            }

            print_close_tag("ifStatement", base_indent);
        }
        Statement::While(conditional, statements) => {
            print_open_tag("whileStatement", base_indent);
            print_element("keyword", "while", base_indent + 1);

            print_element("symbol", "(", base_indent + 1);
            print_expression(conditional, base_indent + 1);
            print_element("symbol", ")", base_indent + 1);

            print_element("symbol", "{", base_indent + 1);
            print_open_tag("statements", base_indent + 1);
            for statement in statements {
                print_statement(statement, base_indent + 2);
            }
            print_close_tag("statements", base_indent + 1);
            print_element("symbol", "}", base_indent + 1);

            print_close_tag("whileStatement", base_indent);
        }
        Statement::Do(maybe_prefix, name, expressions) => {
            print_open_tag("doStatement", base_indent);
            print_element("keyword", "do", base_indent + 1);

            match maybe_prefix {
                Some(prefix) => {
                    print_element("identifier", prefix, base_indent + 1);
                    print_element("symbol", ".", base_indent + 1);
                }
                None => {}
            }

            print_element("identifier", name, base_indent + 1);
            print_element("symbol", "(", base_indent + 1);
            print_expressions(expressions, base_indent + 1);
            print_element("symbol", ")", base_indent + 1);

            print_element("symbol", ";", base_indent + 1);
            print_close_tag("doStatement", base_indent);
        }
        Statement::Return(maybe_expression) => {
            print_open_tag("returnStatement", base_indent);
            print_element("keyword", "return", base_indent + 1);

            match maybe_expression {
                Some(expression) => {
                    print_expression(expression, base_indent + 1)
                }
                None => {}
            }

            print_element("symbol", ";", base_indent + 1);
            print_close_tag("returnStatement", base_indent);
        }
    }
}

fn print_expression(expression: Expression, base_indent: usize) {
    print_open_tag("expression", base_indent);
    print_term(*expression.term, base_indent + 1);

    for (operation, term) in expression.pairs {
        let symbol = match operation {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
            Operation::And => "&",
            Operation::Or => "|",
            Operation::LessThan => "<",
            Operation::GreaterThan => ">",
            Operation::Equals => "=",
        };

        print_element("symbol", escape(symbol.to_string()), base_indent + 1);
        print_term(term, base_indent + 1);
    }

    print_close_tag("expression", base_indent);
}

fn print_term(term: Term, base_indent: usize) {
    print_open_tag("term", base_indent);

    match term {
        Term::IntegerConstant(value) => {
            print_element("integerConstant", value, base_indent + 1);
        }
        Term::StringConstant(value) => {
            print_element("stringConstant", value, base_indent + 1);
        }
        Term::Keyword(keyword_constant) => {
            let value = match keyword_constant {
                KeywordConstant::True => "true",
                KeywordConstant::False => "false",
                KeywordConstant::Null => "null",
                KeywordConstant::This => "this",
            };
            print_element("keyword", value, base_indent + 1);
        }
        Term::Variable(value) => {
            print_element("identifier", value, base_indent + 1);
        }
        Term::ArrayAccess(name, expression) => {
            print_element("identifier", name, base_indent + 1);
            print_element("symbol", "[", base_indent + 1);
            print_expression(expression, base_indent + 1);
            print_element("symbol", "]", base_indent + 1);
        }
        Term::Parenthetical(expression) => {
            print_element("symbol", "(", base_indent + 1);
            print_expression(expression, base_indent + 1);
            print_element("symbol", ")", base_indent + 1);
        }
        Term::Negate(term) => {
            print_element("symbol", "-", base_indent + 1);
            print_term(*term, base_indent + 1);
        }
        Term::Not(term) => {
            print_element("symbol", "~", base_indent + 1);
            print_term(*term, base_indent + 1);
        }
        Term::Call(maybe_class, name, expressions) => {
            match maybe_class {
                Some(class_name) => {
                    print_element("identifier", class_name, base_indent + 1);
                    print_element("symbol", ".", base_indent + 1);
                }
                None => {}
            }
            print_element("identifier", name, base_indent + 1);
            print_element("symbol", "(", base_indent + 1);
            print_expressions(expressions, base_indent + 1);
            print_element("symbol", ")", base_indent + 1);
        }
    }

    print_close_tag("term", base_indent);
}

fn print_data_type(data_type: DataType, base_indent: usize) {
    match data_type {
        DataType::Int => print_element("keyword", "int", base_indent),
        DataType::Char => print_element("keyword", "char", base_indent),
        DataType::Boolean => print_element("keyword", "boolean", base_indent),
        DataType::Class(name) => print_element("identifier", name, base_indent),
    }
}

fn print_parameter(parameter: Parameter, base_indent: usize) {
    print_data_type(parameter.data_type, base_indent);
    print_element("identifier", parameter.name, base_indent);
}

fn print_identifier_list(names: Vec<String>, base_indent: usize) {
    for (index, name) in names.iter().enumerate() {
        if index > 0 {
            print_element("symbol", ",", base_indent);
        }
        print_element("identifier", name, base_indent);
    }
}

fn print_expressions(expressions: Vec<Expression>, base_indent: usize) {
    print_open_tag("expressionList", base_indent);

    for (index, expression) in expressions.into_iter().enumerate() {
        if index > 0 {
            print_element("symbol", ",", base_indent + 1);
        }
        print_expression(expression, base_indent + 1);
    }

    print_close_tag("expressionList", base_indent);
}

fn print_open_tag<S>(name: S, indent_count: usize)
where
    S: std::fmt::Display,
{
    let indent = vec!["  "]
        .into_iter()
        .cycle()
        .take(indent_count)
        .collect::<Vec<&str>>()
        .join("");
    println!("{}<{}>", indent, name);
}

fn print_close_tag<S>(name: S, indent_count: usize)
where
    S: std::fmt::Display,
{
    let indent = vec!["  "]
        .into_iter()
        .cycle()
        .take(indent_count)
        .collect::<Vec<&str>>()
        .join("");
    println!("{}</{}>", indent, name);
}

fn print_element<S, T>(name: S, value: T, indent_count: usize)
where
    S: std::fmt::Display,
    T: std::fmt::Display,
{
    let indent = vec!["  "]
        .into_iter()
        .cycle()
        .take(indent_count)
        .collect::<Vec<&str>>()
        .join("");
    println!("{}<{}> {} </{}>", indent, name, value, name);
}

fn escape(string: String) -> String {
    match string.as_ref() {
        "<" => "&lt;".into(),
        ">" => "&gt;".into(),
        "&" => "&amp;".into(),
        "\"" => "&quot;".into(),
        _ => string,
    }
}
