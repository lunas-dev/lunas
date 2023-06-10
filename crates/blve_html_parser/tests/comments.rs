use insta::assert_json_snapshot;
use blve_html_parser::{Dom, Result};

#[test]
fn it_can_parse_document_with_just_one_comment() -> Result<()> {
    let html = "<!-- hello !\"#/()= -->";
    let ast = Dom::parse(html)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_document_with_just_comments() -> Result<()> {
    let html = "<!--x--><!--y--><!--z-->";
    let ast = Dom::parse(html)?;
    assert_json_snapshot!(ast);
    Ok(())
}
