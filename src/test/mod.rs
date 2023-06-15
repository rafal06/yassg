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
fn inserting_components_with_no_parameters() {
    let components = HashMap::from([
        ("Greeting".to_string(), Component::new( "Greeting".to_string(), "<h1>Hello, World!</h1>".to_string()))
    ]);
    let file = HtmlFile::new(
        "index".to_string(),
        r#"<div><Greeting /></div>"#.to_string(),
    );

    let file_inserted = file.insert_components(&components);
    let expected = r#"<div><h1>Hello, World!</h1></div>"#;

    assert_eq!(file_inserted.content, expected);
}

#[test]
fn inserting_components_with_one_parameter() {
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

#[test]
fn inserting_components_with_multiple_parameters() {
    let components = HashMap::from([
        ("Paragraph".to_string(), Component::new(
            "Paragraph".to_string(),
            "<h{{heading_level}}>{{title}}</h{{heading_level}}><p>{{text}}</p>".to_string()))
    ]);
    let file = HtmlFile::new(
        "index".to_string(),
        r#"<div><Paragraph heading_level="2" title="Lorem" text="Lorem ipsum dolor" /></div>"#.to_string(),
    );

    let file_inserted = file.insert_components(&components);
    let expected = r#"<div><h2>Lorem</h2><p>Lorem ipsum dolor</p></div>"#;

    assert_eq!(file_inserted.content, expected);
}
