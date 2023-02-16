use std::collections::HashMap;
use crate::*;
use crate::html_file::HtmlFile;

#[test]
fn inserting_variables() {
    let component = Component::new(
        "TestComponent".to_string(),
        "<div>Hello, {{name}}! Your name is {{name}}. I'm a {{myName}}</div>".to_string(),
    );
    let vars_str = r#"name="Adam" myName="Component""#;
    let inserted_component = component.insert_variables(vars_str);
    
    let how_it_should_be = "<div>Hello, Adam! Your name is Adam. I'm a Component</div>";
    assert_eq!(inserted_component.content.as_str(), how_it_should_be);
}

#[test]
fn inserting_components() {
    let components = HashMap::from([
        ("Greeting".to_string(), Component::new( "Greeting".to_string(), "<h1>Hello, {{name}}!</h1>".to_string()))
    ]);
    let file = HtmlFile::new(
        "index".to_string(),
        r#"<div><Greeting name="World" /></div>"#.to_string(),
    );

    let file_inserted = file.insert_components(&components);
    let expected = r#"<div><h1>Hello, World!</h1></div>"#;
    
    assert_eq!(file_inserted.content, expected);
}
