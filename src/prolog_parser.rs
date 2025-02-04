use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric1, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub enum PrologClause {
    Fact(PrologAtom),
    Rule(PrologAtom, Vec<PrologAtom>),
}

#[derive(Debug)]
pub struct PrologAtom {
    pub name: String,
    pub terms: Vec<PrologTerm>,
}

#[derive(Debug)]
pub enum PrologTerm {
    Constant(i32),
    Variable(String),
    Compound(String, Vec<PrologTerm>),
}

fn parse_constant(input: &str) -> IResult<&str, PrologTerm> {
    map(take_while1(|c: char| c.is_digit(10)), |s: &str| {
        PrologTerm::Constant(s.parse::<i32>().unwrap())
    })(input)
}

fn parse_variable(input: &str) -> IResult<&str, PrologTerm> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        |s: &str| PrologTerm::Variable(s.to_string()),
    )(input)
}

fn parse_functor(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_alphabetic() || c == '_' || c.is_digit(10)),
        |s: &str| s.to_string(),
    )(input)
}

fn parse_term_list(input: &str) -> IResult<&str, Vec<PrologTerm>> {
    separated_list0(
        delimited(multispace0, tag(","), multispace0),
        parse_term,
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, PrologTerm> {
    let compound_parser = map(
        tuple((
            parse_functor,
            delimited(
                delimited(multispace0, tag("("), multispace0),
                parse_term_list,
                delimited(multispace0, tag(")"), multispace0),
            ),
        )),
        |(name, terms)| PrologTerm::Compound(name, terms),
    );
    alt((parse_constant, parse_variable, compound_parser))(input)
}

fn parse_atom(input: &str) -> IResult<&str, PrologAtom> {
    let compound_parser = map(
        tuple((
            parse_functor,
            delimited(
                delimited(multispace0, tag("("), multispace0),
                parse_term_list,
                delimited(multispace0, tag(")"), multispace0),
            ),
        )),
        |(name, terms)| PrologAtom { name, terms },
    );
    alt((compound_parser, map(parse_term, |t| PrologAtom {
        name: format!("{:?}", t),
        terms: vec![t],
    })))(input)
}

fn parse_fact(input: &str) -> IResult<&str, PrologClause> {
    let (input, atom) = terminated(parse_atom, tuple((multispace0, tag("."), multispace0)))(input)?;
    Ok((input, PrologClause::Fact(atom)))
}

fn parse_rule(input: &str) -> IResult<&str, PrologClause> {
    let (input, head) = terminated(parse_atom, multispace0)(input)?;
    let (input, _) = tag(":-")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = separated_list0(delimited(multispace0, tag(","), multispace0), parse_atom)(input)?;
    let (input, _) = tuple((multispace0, tag("."), multispace0))(input)?;
    Ok((input, PrologClause::Rule(head, body)))
}

fn parse_clause(input: &str) -> IResult<&str, PrologClause> {
    alt((parse_rule, parse_fact))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<PrologClause>> {
    separated_list0(multispace1, parse_clause)(input)
}

#[derive(Debug)]
pub enum Instruction {
    AssertClause { predicate: String, address: usize },
}

pub fn compile_clause(clause: PrologClause) -> Vec<Instruction> {
    match clause {
        PrologClause::Fact(atom) => {
            vec![Instruction::AssertClause {
                predicate: atom.name,
                address: 0,
            }]
        }
        PrologClause::Rule(head, _) => {
            vec![Instruction::AssertClause {
                predicate: head.name,
                address: 0,
            }]
        }
    }
}
