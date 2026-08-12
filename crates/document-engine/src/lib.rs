//! Nexivora Document Engine
//!
//! Provides rich text document model with sections, paragraphs, runs,
//! and serialization support.

pub use types::*;
mod types;

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub id: String,
    pub title: Option<String>,
    pub sections: Vec<types::Section>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            title: None,
            sections: vec![types::Section::default()],
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn add_section(&mut self, section: types::Section) {
        self.sections.push(section);
    }

    pub fn body_section_mut(&mut self) -> Option<&mut types::Section> {
        self.sections.first_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_basics() {
        let mut doc = Document::new();
        doc.title = Some("Test".to_string());

        {
            let section = doc.body_section_mut().unwrap();
            let mut para = types::Paragraph::new("Normal");
            let run = types::Run {
                text: "Hello World".to_string(),
                bold: true,
                ..types::Run::default()
            };
            para.add_run(run);
            section.paragraphs.push(para);
        }

        assert_eq!(doc.title, Some("Test".to_string()));
        assert_eq!(doc.sections[0].paragraphs[0].runs[0].text, "Hello World");
    }
}
