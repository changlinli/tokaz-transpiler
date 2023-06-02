use std::borrow::Borrow;
use std::{env, fs};
use clap::Arg;
use clap::Parser;
use nom::{bytes::complete::{tag, take_while_m_n}, combinator::map_res, sequence::Tuple, IResult, InputTake, FindSubstring, InputLength};
use nom::branch::alt;
use nom::bytes::complete::{take_until, take_while};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::error::ParseError;
use crate::AST::{App, IntegerLiteral};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub enum Type {
    PrimitiveInt,
    PrimitiveNat,
    PrimitiveString,
    PrimitiveBool,
}

pub struct ArgTypePair {
    pub arg_name: Box<str>,
    pub arg_type: Box<str>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionArgument {
    pub arg_name: Box<str>,
    pub arg_type: Box<str>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: Box<str>,
    pub args: Box<[FunctionArgument]>,
    pub return_type: Box<str>,
    pub body: Box<AST>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionApplication {
    pub function_name: Box<str>,
    pub args: Box<[AST]>,
}

#[derive(Debug, PartialEq)]
pub struct ValBinding {
    pub val_name: Box<str>,
    pub val_value: Box<AST>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionBlock {
    pub lines: Box<[AST]>,
    pub final_result: Box<Option<AST>>,
}

#[derive(Debug, PartialEq)]
pub enum AST {
    App(FunctionApplication),
    AppArg(Box<str>),
    TopLevelValBinding(ValBinding),
    FunctionAST(Function),
    IntegerLiteral(i128),
    Empty,
    ExpressionAST(ExpressionBlock),
}

fn parse_val_binding(input: &str) -> IResult<&str, ValBinding> {
    let (input, _) = tag("val")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_until(" ")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, type_name) = take_until("=")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expression_str) = take_until("\n")(input)?;
    let (_, integer_literal) = parse_dec(expression_str)?;
    Ok(
        (
            input,
            ValBinding {
                val_name: name.to_string().into_boxed_str(),
                val_value: Box::new(AST::IntegerLiteral(integer_literal)),
            },
        )
    )
}

fn parse_function(input: &str) -> IResult<&str, Function> {
    let (input, _) = tag("function")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = take_until("(")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = take_until(")")(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, return_type) = take_until(" ")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = tag("}")(input)?;
    Ok(
        (
            input,
            Function {
                name: name.to_string().into_boxed_str(),
                args: Box::new([]),
                return_type: return_type.to_string().into_boxed_str(),
                body: Box::new(AST::Empty),
            },
        )
    )
}

fn parse_ast(input: &str) -> IResult<&str, AST> {
    let parse_as_function_ast = map(parse_function, (|x| AST::FunctionAST(x)));
    let parse_val_binding_ast = map(parse_val_binding, AST::TopLevelValBinding);
    let (input, ast) = alt((parse_as_function_ast, parse_val_binding_ast))(input)?;
    Ok(
        (
            input,
            ast,
        )
    )
}

fn pretty_print_function_application_as_idris(function_name: &str, arguments: &[AST]) -> String {
    let mut result_str = format!("{}", function_name.to_string());
    for arg in arguments {
        result_str.push_str(pretty_print_ast(arg).as_str())
    }
    result_str
}

fn pretty_print_ast(ast: &AST) -> String {
    "some_ast".to_string()
}

fn transform_to_idris(input: &str) -> IResult<&str, String> {
    let (input, result) = parse_ast(input)?;
    Ok(
        (
            input,
            pretty_print_ast(result.borrow()),
        )
    )
}

fn from_dec(input: &str) -> Result<i128, std::num::ParseIntError> {
    i128::from_str_radix(input, 10)
}

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_dec(input: &str) -> IResult<&str, i128> {
    map_res(take_while(is_dec_digit), from_dec)(input)
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str,  u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file_name: String,
}

fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.input_file_name) {
        Ok(contents) => println!("{}", transform_to_idris(&contents).unwrap().1),
        Err(err) => println!("Error reading file: {}", err),
    }
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}

#[test]
fn parse_function_test_simple() {
    assert_eq!(
        parse_function("function myFunction(): Unit {}"),
        Ok((
            "",
            Function {
                name: "myFunction".to_string().into_boxed_str(),
                args: Box::new([]),
                return_type: "Unit".to_string().into_boxed_str(),
                body: Box::new(AST::Empty),
            }
        ))
    )
}

#[test]
fn parse_ast_test_simple() {
    assert_eq!(
        parse_ast("function myFunction(): Unit {}"),
        Ok((
            "",
            AST::FunctionAST(
                Function {
                    name: "myFunction".to_string().into_boxed_str(),
                    args: Box::new([]),
                    return_type: "Unit".to_string().into_boxed_str(),
                    body: Box::new(AST::Empty),
                }
            )
        ))
    )
}

#[test]
fn parse_top_level_val_binding_test() {
    assert_eq!(
        parse_val_binding("val x: Integer = 5\n"),
        Ok(
            (
                "\n",
                ValBinding {
                    val_name: "x".to_string().into_boxed_str(),
                    val_value: Box::new(IntegerLiteral(5)),
                }
            )
        )
    )
}

#[test]
fn parse_integer_literal_test_simple() {
    assert_eq!(
        parse_dec("15"),
        Ok((
            "",
            15,
        ))
    )
}

#[test]
fn parse_function_application_simple() {
    assert_eq!(
        parse_ast("myFunction(5, 1)"),
        Ok((
            "",
            AST::App(
                FunctionApplication {
                    function_name: "myFunction".to_string().into_boxed_str(),
                    args: Box::new([AST::IntegerLiteral(5), AST::IntegerLiteral(1)]),
                }
            )
        ))
    )
}
