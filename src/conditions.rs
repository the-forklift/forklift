use crate::joystick::{Panel, PanelValue};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct WhereClause {
    sub_condition: SubCondition,
    operator: Option<Operator>,
    parameter: PanelValue,
}

impl WhereClause {
    pub fn new(
        sub_condition: SubCondition,
        operator: Option<Operator>,
        parameter: PanelValue,
    ) -> Self {
        Self {
            sub_condition,
            operator,
            parameter,
        }
    }

    pub fn from_parse_types(
        sub_condition: ParseType,
        operator: Option<ParseType>,
        parameter: ParseType,
    ) -> Option<Self> {
        match (sub_condition, operator, parameter) {
            (
                ParseType::SubCondition(subc),
                Some(ParseType::Operator(op)),
                ParseType::Value(param),
            ) => Some(Self::new(subc, Some(op), param)),
            (ParseType::SubCondition(subc), None, ParseType::Value(param)) => {
                Some(Self::new(subc, None, param))
            }
            _ => None,
        }
    }

    pub fn try_from_tokens(tokens: &Panel<'_>) -> Option<Self> {
        let Panel::TokenValue(tokens) = tokens else {
            return None;
        };

        let (clause, stack, count) = tokens.iter().fold(
            (<Option<WhereClause>>::None, VecDeque::new(), 0usize),
            |(mut clause, mut stack, count), word| match (clause, count) {
                (c, 0)
                    if let Some(subc @ SubCondition::Version) =
                        SubCondition::try_from_token(word) =>
                {
                    stack.push_back(ParseType::SubCondition(subc));
                    (c, stack, count + 2)
                }
                (c, 2)
                    if let constraint @ PanelValue::Semver(_) =
                        PanelValue::try_from_token(word) =>
                {
                    stack.push_back(ParseType::Value(constraint));
                    (c, stack, count + 1)
                }

                (None, _) if word == &"and" => {
                    todo!()
                }
                _ => todo!(),
            },
        );

        let (subcondition, operator, value) = match (stack, count) {
            (mut s, 3) => {
                let subcondition = s.pop_front().unwrap();
                let (operator, value) = if (s.len()) == 1 {
                    (None, s.pop_front().unwrap())
                } else {
                    (s.pop_front(), s.pop_front().unwrap())
                };
                s.clear();
                (subcondition, operator, value)
            }
            (s, _) => {
                todo!()
            }
        };

        Self::from_parse_types(subcondition, operator, value)
    }
}
#[derive(Debug, Clone)]
pub enum ParseType {
    SubCondition(SubCondition),
    Operator(Operator),
    Value(PanelValue),
}

impl ParseType {
    pub fn from_tokens(_tokens: &[&str]) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum SubCondition {
    Version,
}

impl SubCondition {
    pub fn try_from_token(input: &str) -> Option<Self> {
        match input {
            "version" => Some(SubCondition::Version),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
}

pub struct PredicateComposition {
    left: Predicate,
    conjunction: Option<Conjunction>,
    right: Option<Predicate>,
}

pub enum Predicate {
    Group(Box<Self>),
    Single(WhereClause),
}

pub enum Conjunction {
    And,
    Or,
}

impl Conjunction {
    pub fn is_valid(conjunction: &str) -> bool {
        conjunction == "and" || conjunction == "AND" || conjunction == "or" || conjunction == "OR"
    }
}
