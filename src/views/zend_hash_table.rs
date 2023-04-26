use super::zval::ZvalView;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::types::{ZendHashTable, Zval};
use liquid::model::{
    ArrayView, DisplayCow, KString, KStringCow, ObjectRender, ObjectSource, State, Value,
};
use liquid::{Object, ObjectView, ValueView};
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct ZendHashTableView<'a>(pub &'a ZendHashTable);

impl<'a> ArrayView for ZendHashTableView<'a> {
    fn as_value(&self) -> &dyn ValueView {
        self
    }

    fn size(&self) -> i64 {
        self.0.len() as i64
    }

    fn values<'k>(&'k self) -> Box<dyn Iterator<Item = &'k dyn ValueView> + 'k> {
        let values = self.0.values().map(|v| convert_value(v));

        Box::new(values)
    }

    fn contains_key(&self, index: i64) -> bool {
        self.0.get_index(index as u64).is_some()
    }

    fn get(&self, index: i64) -> Option<&dyn ValueView> {
        self.0
            .get_index(index as u64)
            .and_then(|v| Some(convert_value(v)))
    }

    fn first(&self) -> Option<&dyn ValueView> {
        self.0.get_index(0).and_then(|v| Some(convert_value(v)))
    }

    fn last(&self) -> Option<&dyn ValueView> {
        self.0
            .get_index(self.0.len() as u64 - 1)
            .and_then(|v| Some(convert_value(v)))
    }
}

fn convert_value(s: &Zval) -> &dyn ValueView {
    let zval = Box::new(ZvalView(s));

    return Box::leak(zval);
}

struct ZendHashTableViewRender<'s> {
    s: &'s ZendHashTableView<'s>,
}

impl<'s> fmt::Display for ZendHashTableViewRender<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, key, value) in self.s.0.iter() {
            write!(f, "{}:{:?}:{:?}", index, key, value)?;
        }
        Ok(())
    }
}

impl<'a> ValueView for ZendHashTableView<'a> {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn render(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ZendHashTableViewRender { s: self }))
    }

    fn source(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(ZendHashTableViewRender { s: self }))
    }

    fn type_name(&self) -> &'static str {
        "array"
    }

    fn query_state(&self, state: State) -> bool {
        match state {
            State::Truthy => true,
            State::DefaultValue | State::Empty | State::Blank => self.0.is_empty(),
        }
    }

    fn to_kstr(&self) -> KStringCow<'_> {
        let s = ZendHashTableViewRender { s: self }.to_string();
        KStringCow::from_string(s)
    }

    fn to_value(&self) -> Value {
        let values = self
            .0
            .iter()
            .map(|(_, _, value)| ZvalView(value).to_value())
            .collect();

        Value::Array(values)
    }

    fn as_array(&self) -> Option<&dyn ArrayView> {
        Some(self)
    }
}

impl<'a> Iterator for ZendHashTableView<'a> {
    type Item = (u64, Option<String>, ZvalView<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .iter()
            .map(|(index, key, value)| (index, key, ZvalView(value)))
            .next()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.len()
    }
}
