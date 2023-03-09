mod display;

use super::Rule;
use itertools::Itertools;
use pest::iterators::Pair;
use std::collections::BTreeSet;

pub struct RonFile(BTreeSet<String>, Box<Value>);

pub struct Value(usize, Kind);

pub enum Kind {
    Atom(String), // atomic types: bool, char, str, int, float, unit type
    List(Vec<Value>),
    Map(Vec<(Value, Value)>),
    TupleType(Option<String>, Vec<Value>),
    FieldsType(Option<String>, Vec<(String, Value)>),
}

impl RonFile {
    pub fn parse_from(pair: Pair<Rule>) -> RonFile {
        assert!(pair.as_rule() == Rule::ron_file, "expected ron_file pair");

        let mut iter = pair.into_inner();
        let extensions = iter
            .take_while_ref(|item| item.as_rule() == Rule::extension)
            .flat_map(Pair::into_inner)
            .map(|ext_name| ext_name.as_str().into())
            .collect();
        let value = iter.next().map(Value::from).unwrap();

        assert!(iter.next().unwrap().as_rule() == Rule::EOI);

        RonFile(extensions, Box::new(value))
    }
}

impl Value {
    fn from(pair: Pair<Rule>) -> Value {
        match pair.as_rule() {
            Rule::bool
            | Rule::char
            | Rule::string
            | Rule::signed_int
            | Rule::float
            | Rule::unit_type => {
                let a = pair.as_str().to_string();
                Value(a.len(), Kind::Atom(a))
            }

            Rule::list => {
                let values: Vec<_> = pair.into_inner().map(Value::from).collect();
                let len = values.iter().map(|n| n.0 + 2).sum(); // N elements -> N-1 ", " + "[]" -> +2 chars per element

                Value(len, Kind::List(values))
            }

            Rule::map => {
                let entries: Vec<_> = pair
                    .into_inner()
                    .map(|entry| {
                        let mut kv_iter = entry.into_inner();
                        let (k, v) = (kv_iter.next().unwrap(), kv_iter.next().unwrap());
                        (Value::from(k), Value::from(v))
                    })
                    .collect();
                let len = entries.iter().map(|(k, v)| k.0 + v.0 + 4).sum(); // N entries -> N ": " + N-1 ", " + "{}" -> +4 chars per entry

                Value(len, Kind::Map(entries))
            }

            Rule::tuple_type => {
                let mut iter = pair.into_inner().peekable();
                let ident = match iter.peek().unwrap().as_rule() {
                    Rule::ident => Some(iter.next().unwrap().as_str().to_string()),
                    _ => None,
                };

                let values: Vec<_> = iter.map(Value::from).collect();
                let len = ident.as_ref().map_or(0, String::len)
                    + values.iter().map(|n| n.0 + 2).sum::<usize>(); // N elements -> N-1 ", " + "()" -> +2 chars per element

                Value(len, Kind::TupleType(ident, values))
            }

            Rule::fields_type => {
                let mut iter = pair.into_inner().peekable();
                let ident = match iter.peek().unwrap().as_rule() {
                    Rule::ident => Some(iter.next().unwrap().as_str().to_string()),
                    _ => None,
                };

                let fields: Vec<_> = iter
                    .map(|field| {
                        let mut kv_iter = field.into_inner();
                        let (k, v) = (kv_iter.next().unwrap(), kv_iter.next().unwrap());
                        (k.as_str().to_string(), Value::from(v))
                    })
                    .collect();
                let len = ident.as_ref().map_or(0, String::len)
                    + fields.iter().map(|(k, v)| k.len() + v.0 + 4).sum::<usize>(); // N fields -> N ": " + N-1 ", " + "()" -> +4 chars per field

                Value(len, Kind::FieldsType(ident, fields))
            }

            Rule::value => Value::from(pair.into_inner().next().unwrap()),

            // handled in other rules
            _ => unreachable!(),
        }
    }
}
