use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct MarkdownWriter {
    content: Vec<String>,
}

impl MarkdownWriter {
    pub fn write(mut self, schema: &Schema) -> Vec<String> {
        self.content.push(format!("# {}", schema.title));
        for (name, property) in &schema.properties {
            // self.content.push(format!("\n## {}\n", name));
            // self.content.push(format!("\n-- {}\n", name));
            self.write_property(name, &property, 2);
        }
        self.content
    }

    pub fn write_property(&mut self, name: &str, property: &Property, level: usize) {
        match property {
            Property::Array(array) => self.write_array_property(name, array, level),
            Property::Object(object) => self.write_object_property(name, object, level),
            Property::String(string) => self.write_string_property(string),
            Property::Integer(integer) => self.write_integer_property(integer),
            Property::Boolean(boolean) => self.write_boolean_property(boolean),
            Property::Reference(reference) => self.write_reference(reference),
        }
    }

    pub fn write_array_property(&mut self, name: &str, property: &ArrayProperty, level: usize) {
        let title = property.title.as_deref().unwrap_or("No title");
        let title_prefix = (0..level).map(|_| "#").collect::<String>();
        self.content
            .push(format!("\n{} {}: {}", title_prefix, name, title));
        self.content.push("To be implemented".to_string());
    }

    pub fn write_object_property(&mut self, name: &str, property: &ObjectProperty, level: usize) {
        // let title = property.title.as_ref().unwrap_or("No");
        let title = property.title.as_deref().unwrap_or("No title");
        let title_prefix = (0..level).map(|_| "#").collect::<String>();
        self.content
            .push(format!("\n{} {}: {}", title_prefix, name, title));

        if let Some(ref description) = property.description {
            self.content.push(description.clone());
        }

        let Some(ref properties) = property.properties else {
            return;
        };

        self.write_properties_table(&properties);
    }

    pub fn write_properties_table(&mut self, properties: &BTreeMap<String, Property>) {
        self.content
            .push(format!("\n|Key|Type|Description|\n|-|-|-|"));
        for (name, property) in properties.iter() {
            let (type_id, description) = match property {
                Property::Array(a) => ("array", a.title.as_deref().unwrap_or("TODO")),
                Property::Object(o) => ("object", o.title.as_deref().unwrap_or("TODO")),
                Property::String(s) => ("string", s.title.as_deref().unwrap_or("TODO")),
                Property::Boolean(b) => ("boolean", b.title.as_deref().unwrap_or("TODO")),
                Property::Integer(i) => ("integer", i.title.as_deref().unwrap_or("TODO")),
                Property::Reference(r) => ("reference", "TODO"),
            };

            self.content
                .push(format!("|{}|{}|{}|", name, type_id, description));
        }
        self.content.push(format!(""));
    }

    pub fn write_string_property(&mut self, property: &StringProperty) {
        if let Some(ref title) = property.title {
            self.content.push(title.to_string());
        }
    }

    pub fn write_integer_property(&mut self, property: &IntegerProperty) {
        if let Some(ref title) = property.title {
            self.content.push(title.to_string());
        }
    }

    pub fn write_boolean_property(&mut self, property: &BooleanProperty) {
        if let Some(ref title) = property.title {
            self.content.push(title.to_string());
        }
    }

    pub fn write_reference(&mut self, reference: &Reference) {
        self.content.push(reference.reference.to_string())
    }
}

#[derive(Deserialize)]
pub struct Schema {
    pub title: String,
    pub properties: BTreeMap<String, Property>,
    // #[serde(rename = "$defs")]
    // pub defs: Option<HashMap<String, ObjectProperty>>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Property {
    #[serde(rename = "array")]
    Array(ArrayProperty),
    #[serde(rename = "object")]
    Object(ObjectProperty),
    #[serde(rename = "string")]
    String(StringProperty),
    #[serde(rename = "integer")]
    Integer(IntegerProperty),
    #[serde(rename = "boolean")]
    Boolean(BooleanProperty),
    #[serde(untagged)]
    Reference(Reference),
}

impl Property {
    pub fn human_type(&self) -> &str {
        match &self {
            Self::Array(_) => "array",
            Self::Object(_) => "object",
            Self::String(_) => "string",
            Self::Integer(_) => "integer",
            Self::Boolean(_) => "boolean",
            Self::Reference(_) => "reference",
        }
    }

    pub fn description(&self) -> &str {
        match &self {
            Self::Array(p) => p.description.as_deref().unwrap_or("TODO"),
            Self::Object(p) => p.description.as_deref().unwrap_or("TODO"),
            Self::String(p) => p.description.as_deref().unwrap_or("TODO"),
            Self::Integer(p) => p.description.as_deref().unwrap_or("TODO"),
            Self::Boolean(p) => p.description.as_deref().unwrap_or("TODO"),
            Self::Reference(_) => "reference",
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PropertyOrReference {
    Property(Property),
    Reference(Reference),
}

#[derive(Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub reference: String,
}

#[derive(Deserialize, Debug)]
pub struct ArrayProperty {
    pub title: Option<String>,
    pub description: Option<String>,
    pub items: Box<PropertyOrReference>,
}

#[derive(Deserialize, Debug)]
pub struct ObjectProperty {
    pub title: Option<String>,
    pub description: Option<String>,
    pub properties: Option<BTreeMap<String, Property>>,
}

#[derive(Deserialize, Debug)]
pub struct StringProperty {
    pub title: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct IntegerProperty {
    pub title: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BooleanProperty {
    pub title: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
}
