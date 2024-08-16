#![warn(clippy::missing_panics_doc, clippy::doc_markdown)]
#![warn(clippy::too_many_lines)]

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

#[derive(Debug)]
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
