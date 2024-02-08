use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use serde_json::{json, Map, Value as JSONValue};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::vec;
use wasm_bindgen::prelude::*;

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    };
}

#[no_mangle]
pub extern "C" fn xtql_json_c(input: *const c_char) -> *const c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = c_str.to_str().unwrap();
    let parsed = parse_xtql(r_str).unwrap().to_string();
    let c_string = CString::new(parsed).unwrap();
    c_string.into_raw()
}

#[wasm_bindgen]
pub fn parse_xtql_to_json(content: &str) -> Result<String, JsValue> {
    match parse_xtql(content) {
        Ok(json_value) => {
            serde_json::to_string(&json_value).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[derive(Parser)]
#[grammar = "xtql.pest"]
pub struct XTQLParser;

pub fn parse_xtql(content: &str) -> Result<JSONValue, Box<pest::error::Error<Rule>>> {
    let xtql = XTQLParser::parse(Rule::Query, content)?.next().unwrap();
    Ok(parse_value(xtql))
}

/// Next 4 functions are helpers to handle the same pattern in the grammar
fn merge_json_objects(vec: Vec<JSONValue>) -> JSONValue {
    let mut merged_map = Map::new();
    for obj in vec {
        if let JSONValue::Object(map) = obj {
            for (key, value) in map {
                merged_map.insert(key, value);
            }
        }
    }
    JSONValue::Object(merged_map)
}

fn flatten_json_array(values: &[JSONValue]) -> Vec<JSONValue> {
    let mut result = Vec::new();
    for value in values {
        match value {
            JSONValue::Array(arr) => {
                result.extend(flatten_json_array(arr));
            }
            _ => result.push(value.clone()),
        }
    }
    result
}

fn extract_xt_param_value(json_value: &JSONValue) -> JSONValue {
    if let JSONValue::Object(obj) = json_value {
        if let Some((_, value)) = obj.iter().next() {
            if let JSONValue::Object(inner_obj) = value {
                if inner_obj.contains_key("xt:param") {
                    return value.clone();
                }
            }
        }
    }
    json_value.clone()
}

/// Can't give the two next functiona a meaningful names, so they are called fn1 and fn2
fn fn1(pair: Pair<Rule>, key: String) -> JSONValue {
    let value = parse_value(pair.clone().into_inner().next().unwrap());
    let args = parse_value(pair.into_inner().nth(1).unwrap());
    merge_json_objects(vec![json!({key: value}), args])
}

fn fn2(pair: Pair<Rule>, key: String) -> JSONValue {
    let mut vec = vec![];
    for inner_pair in pair.into_inner() {
        vec.push(parse_value(inner_pair));
    }
    json!({key: json!(vec) })
}

pub fn parse_value(pair: Pair<Rule>) -> JSONValue {
    match pair.as_rule() {
        // Top Op level
        Rule::Pipeline => {
            let mut vec = vec![];
            for inner_pair in pair.into_inner() {
                vec.push(parse_value(inner_pair));
            }
            json!(json!(vec))
        }
        Rule::Unify => fn2(pair, "unify".to_string()),
        Rule::WithUnify | Rule::WithTail => fn2(pair, "with".to_string()),
        Rule::GroupingVar => {
            json!({ "xt:lvar": parse_value(pair.into_inner().next().unwrap()) })
        }
        Rule::Rel => {
            let mut inner = pair.into_inner();
            let expr = parse_value(inner.next().unwrap());
            let bind_spec = parse_value(inner.next().unwrap());
            json!( { "rel": expr, "bind": bind_spec } )
        }
        Rule::From => {
            let mut inner = pair.into_inner();
            let table = parse_value(inner.next().unwrap());
            let mut bind_specs = parse_value(inner.next().unwrap());
            match &mut bind_specs {
                JSONValue::Object(obj) => {
                    obj.insert("from".to_string(), table);
                    bind_specs
                }
                _ => {
                    json!({ "from": table, "bind": bind_specs })
                }
            }
        }
        Rule::Table => parse_value(pair.into_inner().next().unwrap()),
        Rule::FromOpts => parse_value(pair.into_inner().next().unwrap()),
        Rule::FromOptionVec => parse_value(pair.into_inner().next().unwrap()),
        Rule::FromOptsMap => {
            let mut vec = vec![];
            for inner_pair in pair.into_inner() {
                vec.push(parse_value(inner_pair));
            }
            merge_json_objects(vec)
        }

        Rule::WithTailMap | Rule::WithUnifyMap | Rule::ReturnMap => {
            let mut map = Map::new();
            let mut inner_pairs = pair.into_inner();
            while let Some(key_pair) = inner_pairs.next() {
                if let Some(value_pair) = inner_pairs.next() {
                    let key = parse_value(key_pair).as_str().unwrap().to_string();
                    let value = parse_value(value_pair);
                    map.insert(key, value);
                }
            }
            json!(map)
        }
        Rule::Join => {
            let mut vec = vec![];
            let query = parse_value(pair.clone().into_inner().next().unwrap());
            for inner_pair in pair.into_inner().skip(1) {
                vec.push(parse_value(inner_pair));
            }
            merge_json_objects(vec![json!({"join": query}), json!({"bind": json!(vec)})])
        }
        Rule::LeftJoin => {
            let mut vec = vec![];
            let query = parse_value(pair.clone().into_inner().next().unwrap());
            for inner_pair in pair.into_inner().skip(1) {
                vec.push(parse_value(inner_pair));
            }
            merge_json_objects(vec![
                json!({"leftJoin": query}),
                json!({"bind": json!(vec)}),
            ])
        }
        Rule::OrderBy => fn2(pair, "orderBy".to_string()),
        Rule::OrderBySpecMap => {
            let mut vec = vec![];
            for inner_pair in pair.into_inner() {
                vec.push(parse_value(inner_pair));
            }
            merge_json_objects(vec)
        }
        Rule::OrderBySpecMapVal => {
            let val = parse_value(pair.into_inner().next().unwrap());
            json!({"val": val})
        }
        Rule::OrderBySpecMapDir | Rule::OrderBySpecMapNulls => {
            let value = parse_value(pair.clone().into_inner().next().unwrap());
            let key = if pair.as_rule() == Rule::OrderBySpecMapDir {
                "dir"
            } else {
                "nulls"
            };
            json!({ key: JSONValue::String(value.as_str().unwrap_or("").trim_start_matches(':').to_string()) })
        }
        Rule::OrderByCol => parse_value(pair.into_inner().next().unwrap()),
        Rule::Aggregate => {
            let mut vec = vec![];
            for inner_pair in pair.into_inner() {
                vec.push(parse_value(inner_pair));
            }
            json!({"aggregate": flatten_json_array(vec.as_slice())})
        }
        Rule::GroupingMap => {
            let mut vec = vec![];
            let mut inner_pairs = pair.into_inner();
            while let Some(key_pair) = inner_pairs.next() {
                if let Some(value_pair) = inner_pairs.next() {
                    let key = parse_value(key_pair).as_str().unwrap().to_string();
                    let value = parse_value(value_pair);
                    vec.push(json!({ key: value }));
                }
            }
            json!(vec)
        }
        Rule::Return => fn2(pair, "return".to_string()),
        Rule::ReturnVar => {
            let var = parse_value(pair.into_inner().next().unwrap())
                .as_str()
                .unwrap_or_default()
                .to_string();
            json!({&var: {"xt:lvar": var}})
        }
        Rule::NonNegativeInteger => JSONValue::Number(pair.as_str().parse().unwrap()),
        Rule::AtTempFilter
        | Rule::FromTempFilter
        | Rule::ToTempFilter
        | Rule::ValidTimeKV
        | Rule::SystemTimeKV
        | Rule::Limit
        | Rule::Offset => {
            let key = match pair.as_rule() {
                Rule::AtTempFilter => "at",
                Rule::FromTempFilter => "from",
                Rule::ToTempFilter => "to",
                Rule::ValidTimeKV => "forValidTime",
                Rule::SystemTimeKV => "forSystemTime",
                Rule::Limit => "limit",
                Rule::Offset => "offset",
                _ => unreachable!(),
            };
            json!({ key: parse_value(pair.into_inner().next().unwrap()) })
        }
        Rule::BindKV => {
            json!({ "bind": parse_value(pair.into_inner().next().unwrap())})
        }
        Rule::InTempFilter => fn2(pair, "in".to_string()),

        Rule::EmptyMapExpr => json!({}),
        Rule::EmptyVectorExpr => json!([]),
        Rule::EmptySetExpr => json!({"@typed": "xt:set", "@value": []}),

        Rule::Column | Rule::keyword => JSONValue::String(pair.as_str()[1..].to_string()),
        Rule::MapKey => parse_value(pair.into_inner().next().unwrap()),
        Rule::symbol | Rule::string_content => JSONValue::String(pair.as_str().to_string()),
        Rule::F64 | Rule::I64 => {
            let s = pair.as_str().trim(); // why is this needed?

            if pair.as_rule() == Rule::F64 {
                let num: f64 = s.parse().expect("Could not parse the string as f64");
                json!(num)
            } else {
                let num: i64 = s.parse().expect("Could not parse the string as i64");
                json!(num)
            }
        }
        Rule::Bool => JSONValue::Bool(pair.as_str().parse().unwrap()),
        Rule::Nil => JSONValue::Null,
        Rule::Direction | Rule::NullOrdering => JSONValue::String(pair.as_str().to_string()),
        Rule::String => parse_value(pair.into_inner().next().unwrap()),
        Rule::Function | Rule::LogicVar => parse_value(pair.into_inner().next().unwrap()),

        Rule::CallExpr => {
            let function = parse_value(pair.clone().into_inner().next().unwrap());
            let mut args = vec![];
            for inner_pair in pair.into_inner().skip(1) {
                args.push(parse_value(inner_pair));
            }
            json!({ "xt:call": function, "args": json!(args) })
        }
        Rule::TaggedValueExpr => {
            let mut inner = pair.into_inner();
            let mut tag = parse_value(inner.next().unwrap());
            if let JSONValue::String(tag_str) = &tag {
                tag = match tag_str.as_str() {
                    "time/date" | "instant" => JSONValue::String("xt:date".to_string()),
                    "inst" => JSONValue::String("xt:instant".to_string()),
                    _ => tag,
                };
            }
            let value = parse_value(inner.next().unwrap());
            json!({ "@type": tag, "@value": value })
        }
        Rule::MapExpr | Rule::NonEmptyMapExpr => {
            let rc: Vec<JSONValue> = pair
                .into_inner()
                .map(parse_value)
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|chunk| {
                    let key = chunk[0].as_str().unwrap();
                    let val = &chunk[1];
                    json!({ key: val })
                })
                .collect::<Vec<_>>();
            merge_json_objects(rc)
        }
        Rule::NonEmptyVectorExpr => {
            let rc: Vec<JSONValue> = pair.into_inner().map(parse_value).collect::<Vec<_>>();
            JSONValue::Array(rc)
        }
        Rule::NonEmptySetExpr => {
            let rc: Vec<JSONValue> = pair.into_inner().map(parse_value).collect::<Vec<_>>();
            json!({"@type": "xt:set", "@value": rc})
        }

        // VariableExpression
        Rule::VariableExpr => {
            let var = parse_value(pair.into_inner().next().unwrap());
            json!({ "xt:lvar": var })
        }

        Rule::ParamExpr => {
            let key = {
                let value = parse_value(pair.into_inner().next().unwrap());
                value.as_str().unwrap().to_string()
            };
            json!({&key: { "xt:param": "$".to_string() + &key }})
        }
        Rule::Where => fn2(pair, "where".to_string()),
        Rule::Without => fn2(pair, "without".to_string()),
        Rule::PullExpr => fn1(pair, "xt:pull".to_string()),
        Rule::ExistsExpr => fn1(pair, "xt:exists".to_string()),
        Rule::PullManyExpr => fn1(pair, "xt:pullMany".to_string()),
        Rule::SubqueryExpr => fn1(pair, "xt:q".to_string()),
        Rule::UnnestTail | Rule::UnnestUnify => {
            let expr = parse_value(pair.into_inner().next().unwrap());
            json!({"unnest": expr})
        }
        Rule::UnnestTailSpec | Rule::UnnestUnifySpec => {
            let column = parse_value(pair.clone().into_inner().next().unwrap());
            let expr = parse_value(pair.into_inner().nth(1).unwrap());
            json!({column.as_str().unwrap(): expr})
        }
        // ArgSpec(s). Note the EBNF (doc) doesn't allow for multiple args, but the grammar does
        Rule::ArgSpec => parse_value(pair.into_inner().next().unwrap()),
        Rule::ArgSpecs => {
            let rc: Vec<JSONValue> = pair.into_inner().map(parse_value).collect::<Vec<_>>();
            json!(flatten_json_array(rc.as_slice())) // TODO: useful?
        }
        Rule::BindSpecs => {
            let rc: Vec<JSONValue> = pair.into_inner().map(parse_value).collect();
            let rc = flatten_json_array(rc.as_slice());
            json!(rc)
        }
        Rule::BindVar => {
            let key = {
                let value = parse_value(pair.into_inner().next().unwrap());
                value.as_str().unwrap().to_string()
            };
            json!({&key: { "xt:lvar": &key }})
        }
        Rule::Namespace => parse_value(pair.into_inner().next().unwrap()),
        Rule::NamespacedBindMap => {
            let namespace_value = parse_value(pair.clone().into_inner().next().unwrap());
            let namespace = namespace_value.as_str().unwrap();
            let rc = pair
                .into_inner()
                .skip(1)
                .map(parse_value)
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|chunk| {
                    let key = format!("{}/{}", namespace, chunk[0].as_str().unwrap());
                    let val = &chunk[1];
                    json!({ key: val })
                })
                .collect::<Vec<_>>();
            merge_json_objects(rc)
        }
        Rule::BindMap => {
            let rc: Vec<JSONValue> = pair
                .into_inner()
                .map(parse_value)
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|chunk| {
                    let key = chunk[0].as_str().unwrap();
                    let val = extract_xt_param_value(&chunk[1]);
                    json!({ key: val })
                })
                .collect::<Vec<_>>();
            json!(rc)
        }
        Rule::Args => {
            let args = parse_value(pair.into_inner().next().unwrap());
            json!({ "args": args })
        }
        _ => panic!("Encountered an unexpected rule: {:?}", pair.as_rule()),
    }
}
