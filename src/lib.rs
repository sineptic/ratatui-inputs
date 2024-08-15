#![warn(clippy::missing_panics_doc, clippy::doc_markdown)]

use block_wraper::BlockWrapper;
use ratatui::text::Text;

pub fn get_input(
    input_request: s_text_input_f::Blocks,
    render: &mut impl FnMut(ratatui::text::Text) -> std::io::Result<()>,
) -> Option<std::io::Result<(ResultKind, s_text_input_f::Response)>> {
    let mut blocks_wraper = BlocksWrapper::from(input_request);
    match blocks_wraper.get_input(render)? {
        Ok(result_kind) => Some(Ok((result_kind, blocks_wraper.finalize()))),
        Err(err) => Some(Err(err)),
    }
}

#[derive(Debug)]
pub enum ResultKind {
    Ok,
    Canceled,
    NextItem,
    PrevItem,
}

mod blank_field;

fn split_at_mid<T>(slice: &mut [T], mid: usize) -> Option<(&mut [T], &mut T, &mut [T])> {
    let (head, tail) = slice.split_at_mut(mid);
    let (mid, tail) = tail.split_first_mut()?;
    Some((head, mid, tail))
}

#[derive(Debug)]
struct BlocksWrapper {
    items: Vec<BlockWrapper>,
    cursor: usize,
    start_from_left: bool,
}
impl From<s_text_input_f::Blocks> for BlocksWrapper {
    fn from(value: s_text_input_f::Blocks) -> Self {
        Self {
            items: value.into_iter().map(|x| x.into()).collect(),
            cursor: 0,
            start_from_left: true,
        }
    }
}
impl BlocksWrapper {
    pub fn finalize(self) -> Vec<Vec<String>> {
        self.items.into_iter().map(|x| x.finalize()).collect()
    }
    pub fn get_input(
        &mut self,
        render: &mut impl FnMut(Text) -> std::io::Result<()>,
    ) -> Option<std::io::Result<ResultKind>> {
        // TODO: add support for start from first and from last
        self.select_first_block()?;

        let result_kind = loop {
            let (head, current_block, tail) = split_at_mid(&mut self.items, self.cursor).unwrap();
            let get_input_result = current_block
                .get_input(self.start_from_left, &mut |current_placeholder_lines| {
                    let head_lines = head.iter().flat_map(|x| x.as_lines());
                    let tail_lines = tail.iter().flat_map(|x| x.as_lines());
                    let text: Text = head_lines
                        .chain(current_placeholder_lines)
                        .chain(tail_lines)
                        .collect();
                    render(text)
                })
                .unwrap();
            if let Ok(result_kind) = get_input_result {
                match result_kind {
                    ResultKind::Ok => {
                        let next_elem_exist = self.select_next_block().unwrap();
                        if !next_elem_exist {
                            break ResultKind::Ok;
                        }
                    }
                    ResultKind::Canceled => break ResultKind::Canceled,
                    ResultKind::NextItem => {
                        self.select_next_block().unwrap();
                    }
                    ResultKind::PrevItem => {
                        self.select_prev_block().unwrap();
                    }
                };
            } else {
                return Some(get_input_result);
            }
        };
        Some(Ok(result_kind))
    }
    /// # Errors
    /// if there is no blocks
    fn select_first_block(&mut self) -> Option<()> {
        if self.items.is_empty() {
            None
        } else {
            self.cursor = 0;
            Some(())
        }
    }
    /// # Returns
    /// - `Some(true)`  if next block selected
    /// - `Some(false)` if it's last block already
    /// - `None`        if there is no blocks
    fn select_next_block(&mut self) -> Option<bool> {
        if self.items.is_empty() {
            None
        } else {
            self.cursor += 1;
            if self.cursor < self.items.len() {
                self.start_from_left = true;
                Some(true)
            } else {
                self.cursor -= 1;
                self.start_from_left = false;
                Some(false)
            }
        }
    }
    /// # Returns
    /// - `Some(true)`  if prev block selected
    /// - `Some(false)` if it's first block already
    /// - `None`        if there is no blocks
    fn select_prev_block(&mut self) -> Option<bool> {
        if self.items.is_empty() {
            None
        } else if let Some(x) = self.cursor.checked_sub(1) {
            self.start_from_left = false;
            self.cursor = x;
            Some(true)
        } else {
            self.start_from_left = true;
            Some(false)
        }
    }
}

mod block_wraper {
    use ratatui::text::Line;

    use crate::ResultKind;

    #[derive(Debug)]
    pub enum BlockWrapper {
        Order,
        AnyOf,
        OneOf,
        Paragraph(paragraph_wrapper::ParagraphWrapper),
    }
    impl From<s_text_input_f::Block> for BlockWrapper {
        fn from(value: s_text_input_f::Block) -> Self {
            match value {
                s_text_input_f::Block::Order(_) => todo!(),
                s_text_input_f::Block::AnyOf(_) => todo!(),
                s_text_input_f::Block::OneOf(_) => todo!(),
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
                BlockWrapper::OneOf => todo!(),
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
                BlockWrapper::OneOf => todo!(),
                BlockWrapper::Paragraph(p) => {
                    p.get_input(start_from_left, &mut |line| render(vec![line]))
                }
            }
        }
        pub fn as_lines(&self) -> Vec<Line> {
            match self {
                BlockWrapper::Order => todo!(),
                BlockWrapper::AnyOf => todo!(),
                BlockWrapper::OneOf => todo!(),
                BlockWrapper::Paragraph(p) => vec![p.as_line()],
            }
        }
    }

    mod paragraph_wrapper {
        use paragraph_item_wrapper::*;
        use ratatui::text::Line;

        use crate::{split_at_mid, ResultKind};
        #[derive(Debug)]
        pub struct ParagraphWrapper {
            items: Vec<ParagraphItemWrapper>,
            cursor: usize,
        }
        impl From<s_text_input_f::Paragraph> for ParagraphWrapper {
            fn from(value: s_text_input_f::Paragraph) -> Self {
                Self {
                    items: value.into_iter().map(|x| x.into()).collect(),
                    cursor: 0,
                }
            }
        }
        impl ParagraphWrapper {
            pub fn finalize(self) -> Vec<String> {
                self.items
                    .into_iter()
                    .filter_map(|x| x.finalize().ok())
                    .collect()
            }
            pub fn get_input(
                &mut self,
                start_from_left: bool,
                render: &mut impl FnMut(Line) -> std::io::Result<()>,
            ) -> Option<std::io::Result<ResultKind>> {
                // TODO: add support for start from first and from last

                if start_from_left {
                    self.select_first_placeholder()?;
                } else {
                    self.select_last_placeholder()?;
                }

                let result_kind = loop {
                    let (head, current_placeholder, tail) =
                        split_at_mid(&mut self.items, self.cursor).unwrap();
                    let get_input_result = current_placeholder
                        .get_input(&mut |current_placeholder_spans| {
                            let head_spans = head.iter().flat_map(|x| x.as_spans());
                            let tail_spans = tail.iter().flat_map(|x| x.as_spans());
                            let line: Line = head_spans
                                .chain(current_placeholder_spans)
                                .chain(tail_spans)
                                .collect();
                            render(line)
                        })
                        .unwrap();
                    if let Ok(result_kind) = get_input_result {
                        match result_kind {
                            ResultKind::Ok => {
                                let next_elem_exist = self.select_next_placeholder().unwrap();
                                if !next_elem_exist {
                                    break ResultKind::Ok;
                                }
                            }
                            ResultKind::Canceled => break ResultKind::Canceled,
                            ResultKind::NextItem => {
                                let next_elem_exist = self.select_next_placeholder().unwrap();
                                if !next_elem_exist {
                                    break ResultKind::NextItem;
                                }
                            }
                            ResultKind::PrevItem => {
                                let prev_item_exist = self.select_prev_placeholder().unwrap();
                                if !prev_item_exist {
                                    break ResultKind::PrevItem;
                                }
                            }
                        };
                    } else {
                        return Some(get_input_result);
                    }
                };
                Some(Ok(result_kind))
            }
            /// # Errors
            /// if there is no placeholders
            fn select_first_placeholder(&mut self) -> Option<()> {
                self.cursor = 0;
                if !self.get_current()?.is_placeholder() {
                    let its_wrongly_last = !self.select_next_placeholder()?;
                    if its_wrongly_last {
                        return None;
                    }
                }
                Some(())
            }
            /// # Errors
            /// if there is no placeholders
            fn select_last_placeholder(&mut self) -> Option<()> {
                self.cursor = self.items.len() - 1;
                if !self.get_current()?.is_placeholder() {
                    let its_wrongly_first = !self.select_prev_placeholder()?;
                    if its_wrongly_first {
                        return None;
                    }
                }
                Some(())
            }
            /// # Returns
            /// - `Some(true)`  if next placeholder selected
            /// - `Some(false)` if it's last placeholder already
            /// - `None`        if there is no placeholders
            fn select_next_placeholder(&mut self) -> Option<bool> {
                let starting = self.cursor;
                self.cursor += 1;
                if self.cursor == self.items.len() {
                    self.cursor = starting;
                    return Some(false);
                }
                while (0..(self.items.len() - 1)).contains(&self.cursor)
                    && !self.get_current()?.is_placeholder()
                {
                    self.cursor += 1;
                }
                if self.get_current()?.is_placeholder() {
                    Some(true)
                } else {
                    self.cursor = starting;
                    Some(false)
                }
            }
            /// # Returns
            /// - `Some(true)`  if prev placeholder selected
            /// - `Some(false)` if it's first placeholder already
            /// - `None`        if there is no placeholders
            fn select_prev_placeholder(&mut self) -> Option<bool> {
                let starting = self.cursor;
                self.cursor -= 1;
                if self.cursor == self.items.len() {
                    self.cursor = starting;
                    return Some(false);
                }
                while (1..self.items.len()).contains(&self.cursor)
                    && !self.get_current()?.is_placeholder()
                {
                    self.cursor -= 1;
                }
                if self.get_current()?.is_placeholder() {
                    Some(true)
                } else {
                    self.cursor = starting;
                    Some(false)
                }
            }
            fn get_current(&mut self) -> Option<&mut ParagraphItemWrapper> {
                self.items.get_mut(self.cursor)
            }

            pub fn as_line(&self) -> Line {
                self.items.iter().flat_map(|x| x.as_spans()).collect()
            }
        }

        pub mod paragraph_item_wrapper {
            use crate::{blank_field::BlankField, ResultKind};
            use ratatui::{style::Stylize, text::Span};

            #[derive(Debug)]
            pub enum ParagraphItemWrapper {
                Text(String),
                Placeholder(BlankField),
            }
            impl From<s_text_input_f::ParagraphItem> for ParagraphItemWrapper {
                fn from(value: s_text_input_f::ParagraphItem) -> Self {
                    match value {
                        s_text_input_f::ParagraphItem::Text(s) => Self::Text(s),
                        s_text_input_f::ParagraphItem::Placeholder => {
                            Self::Placeholder(BlankField::default())
                        }
                    }
                }
            }
            impl ParagraphItemWrapper {
                pub fn finalize(self) -> Result<String, Self> {
                    self.try_into_placeholder().map(|x| x.finalize())
                }
                pub fn get_input(
                    &mut self,
                    render: &mut impl FnMut(Vec<Span>) -> std::io::Result<()>,
                ) -> Option<std::io::Result<ResultKind>> {
                    let a = self.as_placeholder()?;
                    Some((|| {
                        Ok(
                            match a.get_input(&mut |c| render_active_blank_field(c, render))? {
                                ResultKind::Ok => ResultKind::Ok,
                                ResultKind::Canceled => ResultKind::Canceled,
                                ResultKind::NextItem => ResultKind::NextItem,
                                ResultKind::PrevItem => ResultKind::PrevItem,
                            },
                        )
                    })())
                }
                // FIXME: rename to as_span, because it can be only 1 span
                pub fn as_spans(&self) -> Vec<Span> {
                    match self {
                        ParagraphItemWrapper::Text(s) => vec![s.into()],
                        ParagraphItemWrapper::Placeholder(blank_field) => {
                            if blank_field.is_empty() {
                                vec![Span::raw("<todo>").dark_gray().italic()]
                            } else {
                                vec![Span::raw(&blank_field.text).underlined().gray().italic()]
                            }
                        }
                    }
                }

                fn try_into_placeholder(self) -> Result<BlankField, Self> {
                    if let Self::Placeholder(v) = self {
                        Ok(v)
                    } else {
                        Err(self)
                    }
                }

                fn as_placeholder(&mut self) -> Option<&mut BlankField> {
                    if let Self::Placeholder(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                /// Returns `true` if the paragraph item wrapper is [`Placeholder`].
                ///
                /// [`Placeholder`]: ParagraphItemWrapper::Placeholder
                #[must_use]
                pub fn is_placeholder(&self) -> bool {
                    matches!(self, Self::Placeholder(..))
                }
            }
            fn render_active_blank_field(
                blank_field: &BlankField,
                render: &mut impl FnMut(Vec<Span>) -> std::io::Result<()>,
            ) -> std::io::Result<()> {
                let (a, b) = blank_field.text.split_at(blank_field.cursor);
                render(vec![
                    Span::raw(a).underlined().italic(),
                    Span::raw("|").blue(),
                    Span::raw(b).underlined().italic(),
                ])
            }
        }
    }
}
