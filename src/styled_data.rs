use crossterm::style::StyledContent;

pub struct StyledData {
    pub styles: Vec<StyledContent<String>>,
    pub len: usize,
}

impl StyledData {
    pub fn from(styles: Vec<StyledContent<String>>, len: usize) -> StyledData {
        StyledData { len, styles }
    }
}
