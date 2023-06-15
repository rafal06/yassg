use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use crate::component::Component;

pub struct HtmlFile {
    pub name: String,
    pub content: String,
}

impl HtmlFile {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
    
    /// Insert all components into a file
    /// 
    /// # Examples
    /// ```rs
    /// let components = Component::get_components(path);
    /// let file = HtmlFile::new(
    ///     "index".to_string(),
    ///     r#"<div>
    ///         <Greeting name="World" />"
    ///     </div>"#.to_string(),
    /// );
    ///
    /// let file_inserted = file.insert_components(&components);
    /// println!("{}", file_inserted.content);
    /// // <div>
    /// //     <h1>Hello, World!</h1>
    /// // </div>
    /// ```
    pub fn insert_components(&self, components: &HashMap<String, Component>) -> Self {
        lazy_static!(
            // 1st capture group: component name
            // 2nd capture group: component parameters
            static ref TAG_RE: Regex = Regex::new(r#"<([A-Z][0-z]*)\s+([0-z]+="[\s\S]*?")*\s*/>"#).unwrap();
        );

        let file_content = TAG_RE.replace_all(&self.content, |caps: &Captures| {
            let comp_name = &caps[1];
            let params = caps.get(2).map_or("", |m| m.as_str());
            
            let comp = match components.get(comp_name) {
                Some(comp) => {
                    log::info!("Inserting component {} into {}", comp_name, &self.name);
                    comp.insert_variables(params)
                },
                None => {
                    log::error!("Invalid component name {} in file {}", comp_name, &self.name);
                    Component::invalid(comp_name)
                },
            };
            
            comp.content
        }).to_string();
        
        Self::new(self.name.clone(), file_content)
    }
}
