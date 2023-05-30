use rome_js_syntax::JsVariableStatement;

fn transform_variable_statement(node: &JsVariableStatement) -> String {
  let declaration = node.declaration()?;
  // let declarators = declaration.declarators().;
  // let declarator = &declarators[0];

  // Get variable name
  // let variable_name = declarator.id().name();

  for item in declaration.declarators().into_iter(){
    let hitem = item?;
  }

  // Get variable value assuming it's a JsStringLiteralExpression
  let variable_value = if let Some(init_clause) = declarator.initializer() {
      if let Some(str_literal_exp) = init_clause.expression().as_string_literal_expression() {
          str_literal_exp.value()
      } else {
          panic!("Initializer is not a string literal!");
      }
  } else {
      panic!("Variable has no initializer!");
  };

  // Transform to new code
  format!("const {} = new valueObj({}, 1, refs);", variable_name, variable_value)
}