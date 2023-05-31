// Identifier

/* "expression": Object {
  "argument": Object {
      "optional": Bool(false),
      "span": Object {
          "ctxt": Number(0),
          "end": Number(44),
          "start": Number(39),
      },
      "type": String("Identifier"),
      "value": String("count"),
  },
  "operator": String("++"),
  "prefix": Bool(false),
  "span": Object {
      "ctxt": Number(0),
      "end": Number(46),
      "start": Number(39),
  },
  "type": String("UpdateExpression"),
}, */

use serde_json::{Map, Value};

use crate::structs::{AddDotV, TransformAnalysisResult, TransformAnalysisResults};

pub fn search_json(
    json: &Value,
    variables: &Vec<String>,
    parent: Option<&Map<String, Value>>,
) -> TransformAnalysisResults {
    if let Value::Object(obj) = json {
        if obj.contains_key("type") && obj["type"] == Value::String("Identifier".into()) {
            if parent.is_some()
                && parent.unwrap().get("type")
                    != Some(&Value::String("VariableDeclarator".to_string()))
            {
                if let Some(Value::String(variable_name)) = obj.get("value") {
                    if variables.iter().any(|e| e == variable_name) {
                        if let Some(Value::Object(span)) = obj.get("span") {
                            if let Some(Value::Number(end)) = span.get("end") {
                                return vec![TransformAnalysisResult::AddDotV(AddDotV {
                                    position: end.as_u64().unwrap() as u32,
                                })];
                            }
                        }
                    }
                }
            }

            return vec![];
        } else {
            let mut result = vec![];
            for (_key, value) in obj {
                let search_result = search_json(value, variables, Some(&obj));
                result.extend(search_result);
            }
            return result;
        }
    } else if let Value::Array(arr) = json {
        let mut result = vec![];
        for value in arr {
            let search_result = search_json(value, variables, None);
            result.extend(search_result);
        }
        return result;
    }
    return vec![];
}

pub fn add_dot_v_to_script(positions: Vec<u32>, script: &String) -> String {
    let mut result = String::new();
    let mut last_position = 0;
    for position in positions {
        result.push_str(&script[last_position..position as usize]);
        result.push_str(".v");
        last_position = position as usize;
    }
    result.push_str(&script[last_position..]);
    return result;
}
