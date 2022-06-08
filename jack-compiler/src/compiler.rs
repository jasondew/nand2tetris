/*
== TODO
- arrays
- strings
- constructors
- method calls
*/

use crate::parser;
use crate::parser::*;
use crate::symbol_table::SymbolTable;
use std::cell::RefCell;

thread_local!( static IF_COUNT: RefCell<u8> = RefCell::new(0));
thread_local!( static WHILE_COUNT: RefCell<u8> = RefCell::new(0));

#[derive(Debug)]
struct Context<'a> {
    class_name: &'a String,
    class_symbol_table: &'a SymbolTable<'a>,
    subroutine_symbol_table: &'a SymbolTable<'a>,
}

pub fn compile<S>(input: S) -> Vec<String>
where
    S: AsRef<str>,
{
    compile_class(parser::parse(input))
}

fn compile_class(class: Class) -> Vec<String> {
    let mut class_symbol_table = SymbolTable::new();
    let mut instructions = vec![];

    for declaration in &class.variable_declarations {
        for name in &declaration.names {
            class_symbol_table.add(
                &name,
                format!("{:?}", &declaration.scope),
                declaration.data_type.clone(),
            );
        }
    }

    for declaration in &class.subroutine_declarations {
        instructions.append(&mut compile_subroutine_declaration(
            declaration,
            &class.name,
            &class_symbol_table,
        ));
    }

    instructions
}

fn compile_subroutine_declaration(
    subroutine_declaration: &SubroutineDeclaration,
    class_name: &String,
    class_symbol_table: &SymbolTable,
) -> Vec<String> {
    let mut symbol_table = SymbolTable::new();

    for parameter in &subroutine_declaration.parameters {
        symbol_table.add(
            &parameter.name,
            "arg".into(),
            parameter.data_type.clone(),
        );
    }

    for declaration in &subroutine_declaration.body.variable_declarations {
        for name in &declaration.names {
            symbol_table.add(name, "var".into(), declaration.data_type.clone());
        }
    }

    let mut instructions = vec![format!(
        "function {}.{} {}",
        class_name,
        subroutine_declaration.name,
        symbol_table.count_of(&"var".into())
    )];
    let context = Context {
        class_name,
        class_symbol_table,
        subroutine_symbol_table: &symbol_table,
    };

    instructions.append(&mut compile_subroutine_body(
        &subroutine_declaration.body,
        &context,
    ));

    instructions
}

fn compile_subroutine_body(
    body: &SubroutineBody,
    context: &Context,
) -> Vec<String> {
    IF_COUNT.with(|if_count| {
        *if_count.borrow_mut() = 0;
    });
    WHILE_COUNT.with(|while_count| {
        *while_count.borrow_mut() = 0;
    });

    let mut instructions = vec![];

    for statement in &body.statements {
        instructions.append(&mut compile_statement(statement, context));
    }

    instructions
}

fn compile_statement(statement: &Statement, context: &Context) -> Vec<String> {
    match statement {
        Statement::Let(name, _maybe_array_index, expression) => {
            //TODO: implement array indexing
            let mut instructions = vec![];

            instructions.append(&mut compile_expression(expression, context));
            instructions
                .push(format!("pop {}", variable_location(name, context)));

            instructions
        }
        Statement::If(conditional, true_statements, false_statements) => {
            let true_label = IF_COUNT
                .with(|if_count| format!("IF_TRUE{}", if_count.borrow()));
            let false_label = IF_COUNT
                .with(|if_count| format!("IF_FALSE{}", if_count.borrow()));
            let end_label = IF_COUNT
                .with(|if_count| format!("IF_END{}", if_count.borrow()));
            IF_COUNT.with(|if_count| {
                *if_count.borrow_mut() += 1;
            });

            let mut instructions = compile_expression(conditional, context);

            instructions.append(&mut vec![
                format!("if-goto {true_label}"),
                format!("goto {false_label}"),
                format!("label {true_label}"),
            ]);

            for statement in true_statements {
                instructions.append(&mut compile_statement(statement, context));
            }

            instructions.append(&mut vec![
                format!("goto {end_label}"),
                format!("label {false_label}"),
            ]);

            for statement in false_statements {
                instructions.append(&mut compile_statement(statement, context));
            }

            instructions.push(format!("label {end_label}"));

            instructions
        }
        Statement::While(conditional, statements) => {
            let loop_label = WHILE_COUNT.with(|while_count| {
                format!("WHILE_EXP{}", while_count.borrow())
            });
            let end_label = WHILE_COUNT.with(|while_count| {
                format!("WHILE_END{}", while_count.borrow())
            });
            WHILE_COUNT.with(|while_count| {
                *while_count.borrow_mut() += 1;
            });

            let mut instructions = vec![format!("label {loop_label}")];

            instructions.append(&mut compile_expression(conditional, context));

            instructions.append(&mut vec![
                "not".into(),
                format!("if-goto {end_label}"),
            ]);

            for statement in statements {
                instructions.append(&mut compile_statement(statement, context));
            }

            instructions.append(&mut vec![
                format!("goto {loop_label}"),
                format!("label {end_label}"),
            ]);

            instructions
        }
        Statement::Do(maybe_prefix, name, expressions) => {
            let mut instructions =
                compile_call(maybe_prefix, name, expressions, context);
            instructions.append(&mut vec!["pop temp 0".into()]);
            instructions
        }
        Statement::Return(maybe_expression) => match maybe_expression {
            Some(expression) => {
                let mut instructions = compile_expression(expression, context);
                instructions.push("return".into());
                instructions
            }
            None => vec!["push constant 0".into(), "return".into()],
        },
    }
}

fn compile_expression(
    expression: &Expression,
    context: &Context,
) -> Vec<String> {
    let mut instructions = compile_term(&expression.term, context);

    for (operation, term) in &expression.pairs {
        instructions.append(&mut compile_term(term, context));
        instructions.push(compile_operation(operation));
    }

    instructions
}

fn compile_term(term: &Term, context: &Context) -> Vec<String> {
    match term {
        Term::IntegerConstant(value) => {
            vec![format!("push constant {value}")]
        }
        Term::StringConstant(_value) => {
            //TODO: implement
            vec![]
        }
        Term::Keyword(keyword_constant) => match keyword_constant {
            KeywordConstant::True => {
                vec!["push constant 0".into(), "not".into()]
            }
            KeywordConstant::False => vec!["push constant 0".into()],
            KeywordConstant::Null => vec!["push constant 0".into()],
            KeywordConstant::This => vec!["push pointer 0".into()],
        },
        Term::Variable(name) => {
            vec![format!("push {}", variable_location(name, context))]
        }
        Term::ArrayAccess(_name, _expression) => {
            //TODO: implement
            vec![]
        }
        Term::Parenthetical(expression) => {
            compile_expression(expression, context)
        }
        Term::Negate(term) => {
            let mut instructions = compile_term(term, context);
            instructions.push("neg".into());
            instructions
        }
        Term::Not(term) => {
            let mut instructions = compile_term(term, context);
            instructions.push("not".into());
            instructions
        }
        Term::Call(maybe_prefix, name, expressions) => {
            compile_call(maybe_prefix, name, expressions, context)
        }
    }
}

fn compile_operation(operation: &Operation) -> String {
    match operation {
        Operation::Add => "add",
        Operation::Subtract => "sub",
        Operation::Multiply => "call Math.multiply 2",
        Operation::Divide => "call Math.divide 2",
        Operation::And => "and",
        Operation::Or => "or",
        Operation::LessThan => "lt",
        Operation::GreaterThan => "gt",
        Operation::Equals => "eq",
    }
    .into()
}

fn compile_call(
    maybe_prefix: &Option<String>,
    name: &String,
    expressions: &Vec<Expression>,
    context: &Context,
) -> Vec<String> {
    let call = match maybe_prefix {
        Some(prefix) => {
            format!("call {}.{} {}", prefix, name, expressions.len())
        }
        None => format!("call {} {}", name, expressions.len()),
    };

    let mut instructions = vec![];

    for expression in expressions {
        instructions.append(&mut compile_expression(expression, context));
    }

    instructions.push(call);

    instructions
}

fn variable_location(name: &String, context: &Context) -> String {
    match context.subroutine_symbol_table.lookup(name) {
        Some(symbol_data) => match symbol_data.kind.as_ref() {
            "var" => format!("local {}", symbol_data.id),
            "arg" => format!("argument {}", symbol_data.id),
            kind => panic!("unknown symbol kind: {kind}"),
        },
        None => panic!("symbol {name} not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_variables() {
        assert_eq!(
            compile(
                "\
class Main {
   function void main() {
      do Output.printInt(1 + (2 * 3));
      return;
   }
}
"
            ),
            vec![
                "function Main.main 0",
                "push constant 1",
                "push constant 2",
                "push constant 3",
                "call Math.multiply 2",
                "add",
                "call Output.printInt 1",
                "pop temp 0",
                "push constant 0",
                "return"
            ]
        );
    }

    #[test]
    fn test_with_variables() {
        assert_eq!(
            compile(
                "\
class Main {
    function void main() {
      var int value;
        do Main.fillMemory(8001, 16, -1); // sets RAM[8001]..RAM[8016] to -1
        let value = Memory.peek(8000);    // reads a value from RAM[8000]
        do Main.convert(value);           // performs the conversion
        return;
    }
}
    "
            ),
            vec![
                "function Main.main 1",
                "push constant 8001",
                "push constant 16",
                "push constant 1",
                "neg",
                "call Main.fillMemory 3",
                "pop temp 0",
                "push constant 8000",
                "call Memory.peek 1",
                "pop local 0",
                "push local 0",
                "call Main.convert 1",
                "pop temp 0",
                "push constant 0",
                "return",
            ]
        );
    }

    #[test]
    fn test_if_and_while_loops() {
        assert_eq!(
            compile(
                "\
class Foo {
    function void convert(int value) {
    	var int mask, position;
    	var boolean loop;
    	
    	let loop = true;
    	while (loop) {
    	    let position = position + 1;
    	    let mask = Main.nextMask(mask);
    	
    	    if (~(position > 16)) {
    	
    	        if (~((value & mask) = 0)) {
    	            do Memory.poke(8000 + position, 1);
       	        }
    	        else {
    	            do Memory.poke(8000 + position, 0);
      	        }    
    	    }
    	    else {
    	        let loop = false;
    	    }
    	}
    	return;
    }
}
"
            ),
            vec![
                "function Foo.convert 3",
                "push constant 0",
                "not",
                "pop local 2",
                "label WHILE_EXP0",
                "push local 2",
                "not",
                "if-goto WHILE_END0",
                "push local 1",
                "push constant 1",
                "add",
                "pop local 1",
                "push local 0",
                "call Main.nextMask 1",
                "pop local 0",
                "push local 1",
                "push constant 16",
                "gt",
                "not",
                "if-goto IF_TRUE0",
                "goto IF_FALSE0",
                "label IF_TRUE0",
                "push argument 0",
                "push local 0",
                "and",
                "push constant 0",
                "eq",
                "not",
                "if-goto IF_TRUE1",
                "goto IF_FALSE1",
                "label IF_TRUE1",
                "push constant 8000",
                "push local 1",
                "add",
                "push constant 1",
                "call Memory.poke 2",
                "pop temp 0",
                "goto IF_END1",
                "label IF_FALSE1",
                "push constant 8000",
                "push local 1",
                "add",
                "push constant 0",
                "call Memory.poke 2",
                "pop temp 0",
                "label IF_END1",
                "goto IF_END0",
                "label IF_FALSE0",
                "push constant 0",
                "pop local 2",
                "label IF_END0",
                "goto WHILE_EXP0",
                "label WHILE_END0",
                "push constant 0",
                "return"
            ]
        );
    }
}
