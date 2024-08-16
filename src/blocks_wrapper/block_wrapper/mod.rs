use crate::ResultKind;
use ratatui::text::Line;

#[derive(Debug)]
pub enum BlockWrapper {
    Order,
    AnyOf,
    OneOf(one_of_wrapper::OneOfWrapper),
    Paragraph(paragraph_wrapper::ParagraphWrapper),
}
impl From<s_text_input_f::Block> for BlockWrapper {
    fn from(value: s_text_input_f::Block) -> Self {
        match value {
            s_text_input_f::Block::Order(_) => todo!(),
            s_text_input_f::Block::AnyOf(_) => todo!(),
            s_text_input_f::Block::OneOf(items) => {
                Self::OneOf(one_of_wrapper::OneOfWrapper::from(items))
            }
            s_text_input_f::Block::Paragraph(p) => {
                Self::Paragraph(paragraph_wrapper::ParagraphWrapper::from(p))
            }
        }
    }
}
impl BlockWrapper {
    pub fn finalize(self) -> Vec<String> {
        match self {
            BlockWrapper::Order => todo!(),
            BlockWrapper::AnyOf => todo!(),
            BlockWrapper::OneOf(o) => o.finalize(),
            BlockWrapper::Paragraph(p) => p.finalize(),
        }
    }
    pub fn get_input(
        &mut self,
        start_from_left: bool,
        render: &mut impl FnMut(Vec<Line>) -> std::io::Result<()>,
    ) -> Option<std::io::Result<ResultKind>> {
        match self {
            BlockWrapper::Order => todo!(),
            BlockWrapper::AnyOf => todo!(),
            BlockWrapper::OneOf(o) => o.get_input(start_from_left, render),
            BlockWrapper::Paragraph(p) => {
                p.get_input(start_from_left, &mut |line| render(vec![line]))
            }
        }
    }
    pub fn as_lines(&self) -> Vec<Line> {
        match self {
            BlockWrapper::Order => todo!(),
            BlockWrapper::AnyOf => todo!(),
            BlockWrapper::OneOf(o) => o.as_lines(),
            BlockWrapper::Paragraph(p) => vec![p.as_line()],
        }
    }
}

mod one_of_wrapper;
mod paragraph_wrapper;
