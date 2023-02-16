use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub content: String,
}

impl Component {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
    
    /// Get a generic component indicating invalid component name
    pub fn invalid(name: &str) -> Self {
        Self::new(
            "invalid".to_string(), 
            format!("<p style=\"background-color: red; color: white;\">Invalid component name: {}</p>", name)
        )
    }
    
    /// Insert variables to the component from a string containing variables
    /// 
    /// # Examples
    /// ```rs
    /// let comp = Component::new(
    ///     "Card".to_string(),
    ///     r#"<div><h2>{{name}}</h2><p>{{description}}</p></div>"#.to_string(),
    /// );
    /// let vars_str = r#"name="Foo" description="Foobar""#;
    /// 
    /// let inserted = comp.insert_variables(vars_str);
    /// println!(inserted.content);  // <div><h2>Foo</h2><p>Foobar</p></div>
    /// ```
    pub fn insert_variables(&self, variables_str: &str) -> Self {
        lazy_static! {
            // 1st capture group: variable name
            // 2nd capture group: variable content
            static ref RE_EXTRACT_VARS: Regex = Regex::new(r#"([0-z]+)="(.*?)""#).unwrap();
        }
        let mut comp_content = self.content.clone();

        for cap in RE_EXTRACT_VARS.captures_iter(variables_str) {
            let pattern = format!("{{{{{}}}}}", &cap[1]);
            comp_content = comp_content.replace(&pattern, &cap[2]);
        }
        
        Self::new(self.name.clone(), comp_content)
    }
    
    /// Get all components from a given directory
    pub fn get_components(path: &Path) -> HashMap<String, Self> {
        let mut components = HashMap::new();
        for file in path.read_dir().expect("Couldn't read components directory") {
            let file = file.unwrap();
            let filename = file.file_name().to_string_lossy().to_string();
            
            if !filename.ends_with(".html") { continue; }
            
            let component_name = filename.replace(".html", "");
            if !component_name.chars().next().unwrap().is_uppercase() {
                println!("Component name \"{}\"  doesn't start with an uppercase letter, ignoring...", &component_name);
            }
            // log::info!("Detected component {}", &component_name);

            if let Ok(mut file) = File::open(file.path()) {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).expect("Failed to read component file");
                
                components.insert(
                    component_name.clone(),
                    Self::new(component_name, file_content)
                );
            }
        }
        components
    }
}
