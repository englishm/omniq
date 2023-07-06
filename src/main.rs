use nom::bytes::complete::take_till;
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

enum SExpr {
    List(Vec<SExpr>),
    Atom(String),
}

fn atom(input: &str) -> IResult<&str, SExpr> {
    let (consumed, rest) = take_till(|c: char| c.is_whitespace())(input)?;
    Ok((rest, SExpr::Atom(consumed.to_string())))
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
    println!("sexpr_output: {sexpr_output}");

    Ok(())
}
