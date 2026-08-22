pub(crate) enum HeaderDrawingPhase {
    TopHeader,
    Value,
    BottomHeader,
}

impl HeaderDrawingPhase {
    // Amount of rows table header has, for example:
    // +-------------+
    // | Description |
    // +-------------+
    const HEADER_ROWS: u8 = 3;

    pub(crate) const ALL: [Self; Self::HEADER_ROWS as usize] =
        [Self::TopHeader, Self::Value, Self::BottomHeader];
}

pub(crate) enum RowDrawingPhase {
    Value,
    BottomHeader,
}

impl RowDrawingPhase {
    // Amount of rows a row takes. It's 1 less than the header rows constant
    // since it uses the above header row to display its result and each
    // of the rows below use the bottom row of the above, like this:
    // +----+
    // | Ex |
    // +----+
    // | hi |
    // +----+
    // | xd |
    // +----+
    const ROW_ROWS: u8 = 2;

    pub(crate) const ROW: [Self; Self::ROW_ROWS as usize] = [Self::Value, Self::BottomHeader];
}
