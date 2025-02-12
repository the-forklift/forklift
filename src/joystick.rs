use crate::conditions::WhereClause;
use crate::download::Ignition;
use anyhow::Result;
use semver::VersionReq;
use std::collections::HashMap;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Query {
    package: Box<str>,
    conditions: Option<WhereClause>,
}

impl Query {
    pub fn parse(self) -> Result<()> {
        let ignition = Ignition::init(self)?;
        let krate = ignition.run()?;
        ignition.process_output();
        Ok(())
    }
}
#[derive(Default, Debug)]
pub struct QueryAccumulator<'a>(HashMap<Button, Panel<'a>>);

impl<'a> QueryAccumulator<'a> {
    pub fn from_input(input: &'a str) -> Self {
        let (mut chars, Some(button), mut collector) = input.split_whitespace().fold(
            (HashMap::<Button, Panel<'a>>::new(), None, vec![]),
            |(mut acc, button, mut collector), token| match (Panel::try_from_keyword(token), button)
            {
                (Some(Panel::Button(kw)), Some(but)) if !collector.is_empty() => {
                    let col = core::mem::take(&mut collector);
                    acc.insert(but, Panel::TokenValue(col));
                    (acc, Some(kw), vec![])
                }
                (Some(Panel::Button(kw)), _) => (acc, Some(kw), collector),
                (None, _) => {
                    collector.push(token);
                    (acc, button, collector)
                }
                _ => unreachable!(),
            },
        ) else {
            todo!()
        };

        if !collector.is_empty() {
            let col = core::mem::take(&mut collector);
            chars.insert(button, Panel::TokenValue(col));
        }

        QueryAccumulator(chars)
    }

    pub fn try_get(&self, key: Button) -> Result<&Panel<'a>, InvalidQueryError> {
        if let Some(entry) = self.0.get(&key) {
            Ok(entry)
        } else {
            Err(InvalidQueryError {})
        }
    }
}

impl TryFrom<QueryAccumulator<'_>> for Query {
    type Error = InvalidQueryError;
    fn try_from(accumulator: QueryAccumulator<'_>) -> Result<Self, InvalidQueryError> {
        let Some(Panel::TokenValue(krate)) = accumulator.0.get(&Button::Lift) else {
            return Err(InvalidQueryError {});
        };

        let conditions = accumulator
            .try_get(Button::Where)
            .map(|clauses| WhereClause::try_from_tokens(clauses).unwrap())
            .ok();

        if krate.len() == 1 {
            Ok(Self {
                package: krate[0].into(),
                conditions,
            })
        } else {
            todo!()
        }
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

#[derive(Clone, Debug)]
pub enum Panel<'a> {
    Button(Button),
    TokenValue(Vec<&'a str>),
}

impl Panel<'_> {
    fn try_from_keyword(keyword: &str) -> Option<Self> {
        Button::try_from_keyword(keyword).map(Self::Button)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Lift,
    Where,
}

impl Button {
    fn try_from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "lift" | "LIFT" => Some(Button::Lift),
            "where" | "WHERE" => Some(Button::Where),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PanelValue {
    Crate(String),
    Semver(VersionReq),
}

impl PanelValue {
    pub fn try_from_token(token: &str) -> Self {
        if let Ok(constraint) = VersionReq::parse(token) {
            Self::Semver(constraint)
        } else {
            Self::Crate(token.to_string())
        }
    }
}
