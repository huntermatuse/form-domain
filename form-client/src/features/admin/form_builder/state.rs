use super::helpers::{default_kind_for_value, default_section, generated_id, shifted_index};
use crate::forms::model::{Question, Section};
use dioxus::prelude::*;

pub static BUILDER_PREFILL: GlobalSignal<Option<BuilderDraft>> = Signal::global(|| None);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Selection {
    Section(usize),
    Question(usize, usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuilderDraft {
    pub title: String,
    pub description_markdown: String,
    pub created_by: String,
    pub sections: Vec<Section>,
}

impl Default for BuilderDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description_markdown: String::new(),
            created_by: String::new(),
            sections: vec![default_section(0)],
        }
    }
}

impl BuilderDraft {
    pub(super) fn push_question_to_last_section(&mut self, kind: &str) -> Selection {
        let section_index = self.sections.len().saturating_sub(1);
        let question_index = self.sections[section_index].questions.len();
        let question = Question {
            question_id: generated_id("question"),
            number: (question_index + 1) as u32,
            title: String::new(),
            required: true,
            kind: default_kind_for_value(kind),
        };
        self.sections[section_index].questions.push(question);
        self.renumber();
        Selection::Question(section_index, question_index)
    }

    pub(super) fn add_section(&mut self) {
        self.sections.push(default_section(self.sections.len()));
        self.renumber();
    }

    pub(super) fn remove_section(&mut self, index: usize) {
        if self.sections.len() > 1 && index < self.sections.len() {
            self.sections.remove(index);
            self.renumber();
        }
    }

    pub(super) fn move_section(&mut self, index: usize, direction: isize) {
        let new_index = shifted_index(index, direction, self.sections.len());
        if new_index != index {
            self.sections.swap(index, new_index);
            self.renumber();
        }
    }

    pub(super) fn remove_question(&mut self, section_index: usize, question_index: usize) {
        if let Some(section) = self.sections.get_mut(section_index) {
            if question_index < section.questions.len() {
                section.questions.remove(question_index);
                self.renumber();
            }
        }
    }

    pub(super) fn move_question(
        &mut self,
        section_index: usize,
        question_index: usize,
        direction: isize,
    ) {
        if let Some(section) = self.sections.get_mut(section_index) {
            let new_index = shifted_index(question_index, direction, section.questions.len());
            if new_index != question_index {
                section.questions.swap(question_index, new_index);
                self.renumber();
            }
        }
    }

    fn renumber(&mut self) {
        for (si, section) in self.sections.iter_mut().enumerate() {
            section.number = (si + 1) as u32;
            for (qi, question) in section.questions.iter_mut().enumerate() {
                question.number = (qi + 1) as u32;
            }
        }
    }
}
