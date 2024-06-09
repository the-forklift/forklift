use anyhow::Result;
use std::collections::HashMap;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Query {
    package: Box<str>,
    conditions: Vec<WhereClauses>,
}

impl Query {
    pub fn parse(&self) -> Result<()> {
        crate::fetch::init(&self.package)
    }
}
#[derive(Default, Debug)]
pub struct QueryAccumulator(HashMap<Button, Panel>);

impl QueryAccumulator {
    pub fn from_input(input: &str) -> Self {
        let chars = input
            .split_whitespace()
            .map_windows(|[keyword, param]| {
                match (
                    Panel::try_from_keyword(keyword),
                    Panel::try_from_keyword(param),
                ) {
                    (Some(Panel::Button(b)), Some(ref param @ Panel::Crate(_))) => {
                        Some((b, param.clone()))
                    }
                    (Some(Panel::Crate(_)), Some(Panel::Button(b))) => None,
                    _ => todo!(),
                }
            })
            .flatten()
            .collect();

        QueryAccumulator(chars)
    }

    fn parse_where(&self) -> Vec<WhereClauses> {
        let Some(conditions) = self.0.get(&Button::Where) else {
            return vec![];
        };
        match conditions {
            _ => todo!(),
        }
    }
}

impl TryFrom<QueryAccumulator> for Query {
    type Error = InvalidQueryError;
    fn try_from(accumulator: QueryAccumulator) -> Result<Self, InvalidQueryError> {
        let Some(krate) = accumulator.0.get(&Button::Lift) else {
            return Err(InvalidQueryError {});
        };

        let conditions = accumulator.parse_where();

        Ok(Self {
            package: krate.param_as_string().into(),
            conditions,
        })
    }
}

#[derive(Clone, Debug)]
pub struct InvalidQueryError {}

impl Error for InvalidQueryError {}

impl Display for InvalidQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidQueryError")
    }
}

#[derive(Debug, Clone)]
struct WhereClauses {
    oredicate: Predicate,
    condition: Condition,
    parameter: Panel,
}

#[derive(Clone, Debug)]
enum Panel {
    Button(Button),
    Crate(String),
    WhereClauses(Vec<WhereClauses>),
}
impl Panel {
    fn try_from_keyword(keyword: &str) -> Option<Self> {
        Button::try_from_keyword(keyword)
            .map(Self::Button)
            .or_else(|| Some(Panel::Crate(keyword.to_string())))
    }

    fn param_as_string(&self) -> &str {
        match self {
            Panel::Crate(s) => s,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Button {
    Lift,
    Where,
}

impl Button {
    fn try_from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "list" | "LIST" => Some(Button::Lift),
            "where" | "WHERE" => Some(Button::Where),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Predicate {
    Version,
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equals,
}
