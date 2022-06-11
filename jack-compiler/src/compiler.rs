use crate::parser;
use crate::parser::*;
use crate::symbol_table::{Kind, SymbolData, SymbolTable};
use std::cell::RefCell;

thread_local!(static IF_COUNT: RefCell<u8> = RefCell::new(0));
thread_local!(static WHILE_COUNT: RefCell<u8> = RefCell::new(0));

#[derive(Debug)]
struct Context<'a> {
    class_name: &'a String,
    class_symbol_table: &'a SymbolTable,
    subroutine_symbol_table: &'a SymbolTable,
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
            let kind = match declaration.scope {
                ClassVariableScope::Field => Kind::Field,
                ClassVariableScope::Static => Kind::Static,
            };

            class_symbol_table.add(
                name.clone(),
                kind,
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

    if subroutine_declaration.kind == SubroutineKind::Method {
        symbol_table.add(
            "this".into(),
            Kind::Argument,
            DataType::Class(class_name.clone()),
        );
    }

    for parameter in &subroutine_declaration.parameters {
        symbol_table.add(
            parameter.name.clone(),
            Kind::Argument,
            parameter.data_type.clone(),
        );
    }

    for declaration in &subroutine_declaration.body.variable_declarations {
        for name in &declaration.names {
            symbol_table.add(
                name.clone(),
                Kind::Variable,
                declaration.data_type.clone(),
            );
        }
    }

    let mut instructions = vec![format!(
        "function {}.{} {}",
        class_name,
        subroutine_declaration.name,
        symbol_table.count_of(Kind::Variable)
    )];
    let context = Context {
        class_name,
        class_symbol_table,
        subroutine_symbol_table: &symbol_table,
    };

    match subroutine_declaration.kind {
        SubroutineKind::Function => {}
        SubroutineKind::Method => {
            instructions.append(&mut vec![
                "push argument 0".into(),
                "pop pointer 0".into(),
            ]);
        }
        SubroutineKind::Constructor => {
            instructions.append(&mut vec![
                format!(
                    "push constant {}",
                    class_symbol_table.non_static_count()
                ),
                "call Memory.alloc 1".into(),
                "pop pointer 0".into(),
            ]);
        }
    }

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
        Statement::Let(name, maybe_array_index, expression) => {
            let mut instructions = vec![];

            match maybe_array_index {
                Some(index_expression) => {
                    instructions.append(&mut compile_expression(
                        index_expression,
                        context,
                    ));
                    instructions.append(&mut vec![
                        format!("push {}", variable_location(name, context)),
                        "add".into(),
                    ]);
                    instructions
                        .append(&mut compile_expression(expression, context));
                    instructions.append(&mut vec![
                        "pop temp 0".into(),
                        "pop pointer 1".into(),
                        "push temp 0".into(),
                        "pop that 0".into(),
                    ]);
                }
                None => {
                    instructions
                        .append(&mut compile_expression(expression, context));
                    instructions.push(format!(
                        "pop {}",
                        variable_location(name, context)
                    ));
                }
            }

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

            if false_statements.is_empty() {
                instructions.push(format!("label {false_label}"));
            } else {
                instructions.append(&mut vec![
                    format!("goto {end_label}"),
                    format!("label {false_label}"),
                ]);

                for statement in false_statements {
                    instructions
                        .append(&mut compile_statement(statement, context));
                }

                instructions.push(format!("label {end_label}"));
            }

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
        Term::StringConstant(string) => {
            let mut instructions = vec![
                format!("push constant {}", string.len()),
                "call String.new 1".into(),
            ];

            for ch in string.chars() {
                instructions.append(&mut vec![
                    format!("push constant {}", ch as u8),
                    "call String.appendChar 2".into(),
                ]);
            }

            instructions
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
        Term::ArrayAccess(name, index_expression) => {
            let mut instructions =
                compile_expression(index_expression, context);
            instructions.append(&mut vec![
                format!("push {}", variable_location(name, context)),
                "add".into(),
            ]);
            instructions.append(&mut vec![
                "pop pointer 1".into(),
                "push that 0".into(),
            ]);
            instructions
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
    let mut instructions = vec![];

    let call = match maybe_prefix {
        Some(class_or_variable) => {
            if class_or_variable.starts_with(char::is_uppercase) {
                format!(
                    "call {}.{} {}",
                    class_or_variable,
                    name,
                    expressions.len()
                )
            } else {
                instructions.push(format!(
                    "push {}",
                    variable_location(class_or_variable, context)
                ));
                format!(
                    "call {}.{} {}",
                    class_of(class_or_variable, context),
                    name,
                    expressions.len() + 1
                )
            }
        }
        None => {
            instructions.push("push pointer 0".into());
            format!(
                "call {}.{} {}",
                context.class_name,
                name,
                expressions.len() + 1
            )
        }
    };

    for expression in expressions {
        instructions.append(&mut compile_expression(expression, context));
    }

    instructions.push(call);

    instructions
}

fn variable_location(name: &String, context: &Context) -> String {
    let symbol_data = get_symbol_data(name, context);

    match symbol_data.kind {
        Kind::Variable => format!("local {}", symbol_data.id),
        Kind::Argument => format!("argument {}", symbol_data.id),
        Kind::Field => format!("this {}", symbol_data.id),
        Kind::Static => format!("static {}", symbol_data.id),
    }
}

fn class_of(name: &String, context: &Context) -> String {
    match &get_symbol_data(name, context).data_type {
        DataType::Class(class_name) => class_name.clone(),
        other_type => {
            panic!("expected {name} to have Class type, but was {other_type:?}")
        }
    }
}

fn get_symbol_data<'a>(
    name: &'a String,
    context: &'a Context,
) -> &'a SymbolData {
    match context.subroutine_symbol_table.lookup(name) {
        Some(symbol_data) => symbol_data,
        None => match context.class_symbol_table.lookup(name) {
            Some(symbol_data) => symbol_data,
            None => panic!("symbol {name} not found"),
        },
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

    #[test]
    fn test_method_calls() {
        assert_eq!(
            compile(
                "\
class Main {
    function void main() {
        var SquareGame game;
        let game = SquareGame.new();
        do game.run();
        do game.dispose();
        return;
    }
}
"
            ),
            vec![
                "function Main.main 1",
                "call SquareGame.new 0",
                "pop local 0",
                "push local 0",
                "call SquareGame.run 1",
                "pop temp 0",
                "push local 0",
                "call SquareGame.dispose 1",
                "pop temp 0",
                "push constant 0",
                "return"
            ]
        )
    }

    #[test]
    fn test_constructor_and_class_vars() {
        assert_eq!(
            compile(
                "\
class Square {
   field int x, y;
   field int size;

   constructor Square new(int Ax, int Ay, int Asize) {
      let x = Ax;
      let y = Ay;
      let size = Asize;
      do draw();
      return this;
   }
}
"
            ),
            vec![
                "function Square.new 0",
                "push constant 3",
                "call Memory.alloc 1",
                "pop pointer 0",
                "push argument 0",
                "pop this 0",
                "push argument 1",
                "pop this 1",
                "push argument 2",
                "pop this 2",
                "push pointer 0",
                "call Square.draw 1",
                "pop temp 0",
                "push pointer 0",
                "return"
            ]
        )
    }

    #[test]
    fn test_methods() {
        assert_eq!(
            compile(
                "\
class Square {
   method void dispose() {
      do Memory.deAlloc(this);
      return;
   }
}
"
            ),
            vec![
                "function Square.dispose 0",
                "push argument 0",
                "pop pointer 0",
                "push pointer 0",
                "call Memory.deAlloc 1",
                "pop temp 0",
                "push constant 0",
                "return"
            ]
        )
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            compile(
                "\
class Main {
  function void main() {
    var int length;
    let length = Keyboard.readInt(\"count? \");
    return;
  }
}
"
            ),
            vec![
                "function Main.main 1",
                "push constant 7",
                "call String.new 1",
                "push constant 99",
                "call String.appendChar 2",
                "push constant 111",
                "call String.appendChar 2",
                "push constant 117",
                "call String.appendChar 2",
                "push constant 110",
                "call String.appendChar 2",
                "push constant 116",
                "call String.appendChar 2",
                "push constant 63",
                "call String.appendChar 2",
                "push constant 32",
                "call String.appendChar 2",
                "call Keyboard.readInt 1",
                "pop local 0",
                "push constant 0",
                "return"
            ]
        )
    }

    #[test]
    fn test_arrays() {
        assert_eq!(
            compile(
                "\
class Main {
   function void main() {
     var Array a;
     var int i, sum;

     let a = Array.new(3);
     let i = 0;

     while (i < 3) {
        let a[i] = i;
        let sum = sum + a[i];
        let i = i + 1;
     }

     return;
   }
}
"
            ),
            vec![
                "function Main.main 3",
                "push constant 3",
                "call Array.new 1",
                "pop local 0",
                "push constant 0",
                "pop local 1",
                "label WHILE_EXP0",
                "push local 1",
                "push constant 3",
                "lt",
                "not",
                "if-goto WHILE_END0",
                "push local 1",
                "push local 0",
                "add",
                "push local 1",
                "pop temp 0",
                "pop pointer 1",
                "push temp 0",
                "pop that 0",
                "push local 2",
                "push local 1",
                "push local 0",
                "add",
                "pop pointer 1",
                "push that 0",
                "add",
                "pop local 2",
                "push local 1",
                "push constant 1",
                "add",
                "pop local 1",
                "goto WHILE_EXP0",
                "label WHILE_END0",
                "push constant 0",
                "return"
            ]
        )
    }
}
