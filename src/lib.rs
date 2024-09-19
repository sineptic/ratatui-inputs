#![warn(clippy::missing_panics_doc, clippy::doc_markdown)]
#![warn(clippy::too_many_lines)]

use blocks_wrapper::block_wrapper::paragraph_wrapper::paragraph_item_wrapper::style_active_blank_field;
use s_text_input_f::Block;

pub fn get_input(
    input_request: s_text_input_f::Blocks,
    render: &mut impl FnMut(ratatui::text::Text) -> std::io::Result<()>,
) -> Option<std::io::Result<(ResultKind, s_text_input_f::Response)>> {
    let mut blocks_wraper = blocks_wrapper::BlocksWrapper::from(input_request);
    match blocks_wraper.get_input(render)? {
        Ok(result_kind) => Some(Ok((result_kind, blocks_wraper.finalize()))),
        Err(err) => Some(Err(err)),
    }
}

// TODO: Create custom handled for end of iteraction
#[allow(clippy::missing_panics_doc)]
pub fn display_answer(
    input_blocks: s_text_input_f::Blocks,
    user_answer: Vec<Vec<String>>,
    correct_answer: Vec<Vec<String>>,
    render: &mut impl FnMut(ratatui::text::Text) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let answered = {
        let mut temp = s_text_input_f::to_answered(input_blocks, user_answer, correct_answer)
            .into_iter()
            .map(Block::Answered)
            .collect::<Vec<_>>();
        temp.push(Block::Paragraph(vec![
            s_text_input_f::ParagraphItem::Placeholder,
        ]));
        temp
    };

    get_input(answered, render)
        .expect("last elem must be blank field by design. implementation issue")
        .map(|_| ())
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResultKind {
    Ok,
    Canceled,
    NextBlock,
    PrevBlock,
}

mod blank_field;

fn split_at_mid<T>(slice: &mut [T], mid: usize) -> Option<(&mut [T], &mut T, &mut [T])> {
    let (head, tail) = slice.split_at_mut(mid);
    let (mid, tail) = tail.split_first_mut()?;
    Some((head, mid, tail))
}

mod blocks_wrapper;

pub fn get_text_input(
    render: &mut impl FnMut(ratatui::text::Text, &str) -> std::io::Result<()>,
) -> std::io::Result<(ResultKind, String)> {
    let mut blank_field = blank_field::BlankField::default();
    loop {
        match blank_field.get_input(&mut |blank_field| {
            let styled = ratatui::text::Text::from(ratatui::text::Line::from(
                style_active_blank_field(blank_field),
            ));
            render(styled, blank_field.text())
        })? {
            ResultKind::Ok => return Ok((ResultKind::Ok, blank_field.text().to_owned())),
            ResultKind::Canceled => return Ok((ResultKind::Ok, blank_field.text().to_owned())),
            ResultKind::NextBlock => (),
            ResultKind::PrevBlock => (),
        }
    }
}
