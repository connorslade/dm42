/// Iterator that returns free private identifiers.
///
/// 00-99, A-J, a-e
pub struct FreeIdent {
    idx: u8,
}

impl FreeIdent {
    pub fn new() -> Self {
        Self { idx: 0 }
    }
}

impl Iterator for FreeIdent {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        Some(match idx {
            0..=99 => format!("{:02}", idx),
            100..=109 => ((idx - 100 + b'A') as char).to_string(),
            110..=114 => ((idx - 110 + b'a') as char).to_string(),
            _ => return None,
        })
    }
}
