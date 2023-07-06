use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::multi::separated_list0;
use nom::IResult;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_sexpr;
use std::error;

#[derive(Deserialize, Serialize)]
struct ExampleStruct {
    children: Vec<ExampleStruct>,
    data: Vec<u8>,
}

#[derive(Debug)]
enum SExpr {
    List(Vec<SExpr>),
    Atom(String),
}

fn atom(input: &str) -> IResult<&str, SExpr> {
    let (input, consumed) = take_till(|c: char| c.is_whitespace() || c == ')')(input)?;
    Ok((input, SExpr::Atom(consumed.to_string())))
}

fn list(input: &str) -> IResult<&str, SExpr> {
    let (input, _) = tag("(")(input)?;
    let (input, list) = separated_list0(tag(" "), sexpr)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, SExpr::List(list)))
}

fn sexpr(input: &str) -> IResult<&str, SExpr> {
    alt((list, atom))(input)
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let example_struct = ExampleStruct {
        children: vec![ExampleStruct {
            children: vec![],
            data: vec![],
        }],
        data: vec![1, 2, 3],
    };

    let json_output = serde_json::to_string(&example_struct)?;
    println!("json_output: {json_output}");
    let sexpr_output = serde_sexpr::to_string(&example_struct)?;
    println!("sexpr_output: {}", sexpr_output.clone());

    let (rest, parsed_sexpr) = sexpr(sexpr_output.as_str()).unwrap();
    dbg!(rest, parsed_sexpr);

    Ok(())
}
