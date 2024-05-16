use std::collections::HashMap;

pub struct Query {
    krate: Box<str>,
}

#[derive(Default)]
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
                    (Some(Panel::Button(b)), Some(ref param @ Panel::Crate(ref c))) => {
                        (b, param.clone())
                    }
                    _ => todo!(),
                }
            })
            .collect();

        QueryAccumulator(chars)
    }
}

impl TryFrom<QueryAccumulator> for Query {
    type Error = InvalidQueryError;
    fn try_from(accumulator: QueryAccumulator) -> Result<Self, InvalidQueryError> {
        let Some(krate) = accumulator.0.get(&Button::List) else {
            return Err(InvalidQueryError {});
        };
        Ok(Self {
            krate: krate.param_as_string().into(),
        })
    }
}

pub struct InvalidQueryError {}

#[derive(Clone, Debug)]
enum Panel {
    Button(Button),
    Crate(String),
}

impl Panel {
    fn try_from_keyword(keyword: &str) -> Option<Self> {
        Button::try_from_keyword(keyword)
            .map(Self::Button)
            .or_else(|| None)
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
    List,
}

impl Button {
    fn try_from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "list" | "LIST" => Some(Button::List),
            _ => None,
        }
    }
}
