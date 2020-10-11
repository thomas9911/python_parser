use pest::{error, Parser};
use pest_derive::Parser;

///
/// Bundle
///   Document
///   Document
///     Part
///       Block
///       Block
///         Line
///           Item
///         Line
///           Item
///

#[derive(Debug)]
pub enum Error {
    Pest(error::Error<Rule>),
    Custom(String),
    Empty,
    Grammar(&'static str),
}

impl From<error::Error<Rule>> for Error {
    fn from(err: error::Error<Rule>) -> Error {
        Error::Pest(err)
    }
}

pub fn print_pest_error(error: Error) {
    match error {
        Error::Pest(error) => println!("{}", error),
        // _ => unreachable!(),
        a => {
            println!("{:?}", a);
            unreachable!()
        }
    }
}

// type Error = error::Error<Rule>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

pub fn parse(input: &str) -> Result<Document> {
    Document::parse(input)
}

pub struct Bundle {
    documents: Vec<Document>,
}

impl Bundle {}

#[derive(Debug, PartialEq)]
pub struct Document {
    parts: Vec<Part>,
    location: Option<String>,
}

impl Document {
    pub fn parse(input: &str) -> Result<Document> {
        let pairs = Grammar::parse(Rule::python, input)?;
        Self::lexer(pairs)
    }

    pub fn lexer(pairs: pest::iterators::Pairs<Rule>) -> Result<Document> {
        let mut parts = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::part => {
                    let part = Part::lexer(pair.into_inner())?;
                    parts.push(part)
                }
                Rule::EOI => {
                    return Ok(Document {
                        parts,
                        location: None,
                    })
                }
                // _ => unreachable!()
                a => {
                    println!("{:?}", a);
                    unreachable!()
                }
            }
        }

        Err(Error::Grammar("doc"))
    }
}

#[derive(Debug, PartialEq)]
pub enum Part {
    Class(Class),
    Literal(Block),
}

impl Part {
    pub fn lexer(pairs: pest::iterators::Pairs<Rule>) -> Result<Part> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::block => {
                    let block = Block::lexer(&mut pair.into_inner())?;
                    return Ok(Part::Literal(block));
                }
                a => {
                    println!("{:?}", a);
                    unreachable!()
                }
            }
        }
        // let x = Block::lexer(&mut pairs);
        // println!("{:?}", x);

        Err(Error::Grammar("part"))
    }
}

// #[derive(Debug, PartialEq)]
// pub struct Part {
//     blocks: Vec<Block>,
// }

// impl Part {
//     pub fn parse(input: &str) -> Result<Part> {
//         let pairs = Grammar::parse(Rule::python, input)?;
//         Self::lexer(pairs)
//     }

//     pub fn lexer(mut pairs: pest::iterators::Pairs<Rule>) -> Result<Part> {
//         let mut indent = 0;
//         let mut block_indent;
//         let mut blocks = Vec::new();

//         while let Ok(block) = Block::lexer(&mut pairs) {
//             block_indent = block.indent;
//             blocks.push(block);

//             if (indent > 0) && (block_indent == 0) {
//                 return Ok(Part { blocks });
//             };

//             indent = block_indent;
//         }
//         Ok(Part { blocks })

//         // Err(Error::Grammar("a"))
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Block {
    indent: usize,
    lines: Vec<Line>,
}

impl Block {
    pub fn lexer(pairs: &mut pest::iterators::Pairs<Rule>) -> Result<Block> {
        let mut indent = None;
        let mut lines = Vec::new();

        // if let Ok(line) = Line::lexer(&mut pairs) {
        //     let this_indent = line.indent;
        //     lines.push(line);
        // } else {
        //     return Err(Error::Grammar("a"))
        // }

        while let Some(x) = pairs.peek() {
            if x.as_rule() == Rule::EOI {
                return Ok(Block {
                    indent: indent.unwrap_or(0),
                    lines,
                });
            }
            let mut line = Line::lexer(pairs)?;
            if new_block_indent(indent, line.indent) {
                indent = Some(line.indent);
            };
            if let Some(x) = indent {
                line.indent = x;
            }
            lines.push(line);
        }

        if lines.is_empty() {
            Err(Error::Grammar("block"))
        } else {
            Ok(Block {
                indent: indent.unwrap_or(0),
                lines,
            })
        }
    }

    pub fn empty() -> Block {
        Block {
            indent: 0,
            lines: Vec::new(),
        }
    }
}

fn new_block_indent(a: Option<usize>, b: usize) -> bool {
    if a == None {
        true
    } else if a == Some(b) {
        false
    } else {
        false
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    indent: usize,
    item: Item,
}

impl Line {
    pub fn lexer(pairs: &mut pest::iterators::Pairs<Rule>) -> Result<Line> {
        let mut indent = 0;

        // let mut line = Line {
        //     item: Item::Grammar("a"),
        //     indent: 0,
        // };

        for pair in pairs {
            // println!("{:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::indent => {
                    indent += 1;
                    continue;
                }
                Rule::item => {
                    let item = Item::lexer(pair.into_inner())?;
                    // println!("{:?}", item);
                    // line = Line { indent, item };
                    return Ok(Line { indent, item });
                }
                Rule::newline => continue,
                a => {
                    println!("line: {:?}", a);
                    unreachable!()
                }
            }
        }

        Err(Error::Grammar("line"))
    }
}

#[derive(Debug, PartialEq)]
pub enum Item {
    Empty,
    Variable(String),
    // Classname(String),
    Class(Class),
    Function(Function),
}

impl Item {
    pub fn lexer(pairs: pest::iterators::Pairs<Rule>) -> Result<Item> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::variable => return Ok(Item::Variable(pair.as_str().to_owned())),
                Rule::class => {
                    let class = Class::lexer(pair.into_inner())?;
                    return Ok(Item::Class(class));
                }
                Rule::function => {
                    let function = Function::lexer(pair.into_inner())?;
                    return Ok(Item::Function(function));
                }
                a => {
                    println!("item: {:?}", a);
                    unreachable!()
                } // _ => unreachable!(),
            }
        }

        Err(Error::Grammar("item"))
    }
}

#[derive(Debug, PartialEq)]
pub struct Class {
    name: String,
    block: Block,
}

impl Class {
    pub fn lexer(pairs: pest::iterators::Pairs<Rule>) -> Result<Class> {
        let mut name = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::classname => {
                    name = pair.as_str().to_owned();
                }
                Rule::block => {
                    let block = Block::lexer(&mut pair.into_inner())?;
                    return Ok(Class { block, name });
                }
                Rule::newline => continue,
                a => {
                    println!("class: {:?}", a);
                    unreachable!()
                } // _ => unreachable!(),
            }
        }

        Err(Error::Grammar("class"))
    }
}

#[derive(Debug, PartialEq)]
pub struct Function {
    name: String,
    block: Block,
    input_variables: Variables,
}

impl Function {
    pub fn lexer(pairs: pest::iterators::Pairs<Rule>) -> Result<Function> {
        let mut name = String::new();
        let mut input_variables = Variables::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::functionname => {
                    name = pair.as_str().to_owned();
                }
                Rule::arguments => {
                    input_variables = Variables::lexer(pair.into_inner())?;
                }
                Rule::block => {
                    let block = Block::lexer(&mut pair.into_inner())?;
                    return Ok(Function {
                        block,
                        name,
                        input_variables,
                    });
                }
                Rule::newline => continue,
                a => {
                    println!("function: {:?}", a);
                    unreachable!()
                } // _ => unreachable!(),
            }
        }

        Err(Error::Grammar("function"))
    }
}

#[derive(Debug, PartialEq)]
pub struct Variables {
    variables: Vec<String>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            variables: Vec::new(),
        }
    }

    pub fn lexer(pairs: pest::iterators::Pairs<Rule>) -> Result<Variables> {
        let mut variables = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::variable => variables.push(pair.as_str().to_owned()),
                a => {
                    println!("variables: {:?}", a);
                    unreachable!()
                } // _ => unreachable!(),
            }
        }

        Ok(Variables { variables })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let expected = Document {
            parts: vec![Part::Literal(Block {
                lines: vec![Line {
                    indent: 1,
                    item: Item::Variable("abcd".to_string()),
                }],
                indent: 1,
            })],
            location: None,
        };

        let output = parse("  abcd\n").unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn multiline() {
        let expected = Document {
            parts: vec![Part::Literal(Block {
                lines: vec![
                    Line {
                        indent: 1,
                        item: Item::Variable("abcd".to_string()),
                    },
                    Line {
                        indent: 1,
                        item: Item::Variable("bcdfe".to_string()),
                    },
                ],
                indent: 1,
            })],
            location: None,
        };

        let output = parse("  abcd\n  bcdfe\n").unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn class_1() {
        let input = "
class Test:
  test
  t123  
";

        let expected = Document {
            parts: vec![Part::Literal(Block {
                indent: 0,
                lines: vec![Line {
                    indent: 0,
                    item: Item::Class(Class {
                        name: "Test".to_string(),
                        block: Block {
                            indent: 1,
                            lines: vec![
                                Line {
                                    indent: 1,
                                    item: Item::Variable("test".to_string()),
                                },
                                Line {
                                    indent: 1,
                                    item: Item::Variable("t123".to_string()),
                                },
                            ],
                        },
                    }),
                }],
            })],
            location: None,
        };

        let output = parse(input).unwrap();

        // println!("{:?}", a);
        // match a {
        //     Err(x) => {
        //         print_pest_error(x);
        //         panic!("no")
        //     }
        //     Ok(x) => x,
        // };

        assert_eq!(expected, output);
    }

    #[test]
    fn function_1() {
        let input = "
def test(a, b):
  a
";

        let expected = Document {
            parts: vec![Part::Literal(Block {
                indent: 0,
                lines: vec![Line {
                    indent: 0,
                    item: Item::Function(Function {
                        name: "test".to_string(),
                        block: Block {
                            indent: 1,
                            lines: vec![Line {
                                indent: 1,
                                item: Item::Variable("a".to_string()),
                            }],
                        },
                        input_variables: Variables {
                            variables: vec!["a".to_string(), "b".to_string()],
                        },
                    }),
                }],
            })],
            location: None,
        };

        let output = parse(input).unwrap();

        assert_eq!(expected, output);
    }

    //     #[test]
    //     fn multiline_multiblock() {
    //         let expected = Part {
    //             blocks: vec![
    //                 Block {
    //                     lines: vec![
    //                         Line {
    //                             indent: 1,
    //                             item: Item::Variable("abcd".to_string()),
    //                         },
    //                         Line {
    //                             indent: 1,
    //                             item: Item::Variable("bcdfe".to_string()),
    //                         },
    //                     ],
    //                     indent: 1,
    //                 },
    //                 Block::empty(),
    //             ],
    //         };

    //         Part {
    //             blocks: vec![Block {
    //                 indent: 0,
    //                 lines: vec![Line {
    //                     indent: 0,
    //                     item: Item::Variable("abc".to_string()),
    //                 }],
    //             }],
    //         };

    //         let text = "abc
    //   defg

    // abc
    // ";

    //         let a = parse(text).unwrap();

    //         assert_eq!(expected, a);
    //     }

    // #[test]
    // fn stack_test() {
    //     let input = "class zero:\n  one\n";
    //     let a = parse(input);
    //     let x = match a {
    //         Err(x) => {
    //             print_pest_error(x);
    //             panic!("no")
    //         }
    //         Ok(x) => x,
    //     };

    //     println!("{:?}", x);

    //     assert!(false)
    // }
}
