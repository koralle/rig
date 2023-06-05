use crossterm::event::KeyCode;

#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    ConstructMainView,
    ConstructDiffView,
    ConstructLogView,
    ConstructPagerView,
    ConstructTreeView,
    ConstructBlobView,
    ConstructGrepView,
    ConstructBlameView,
    ConstructRefsView,
    ConstructStashView,
    ConstructHelpView,
    ConstructStatusView,
    ConstructStageView,
    InvalidRequest,
}

impl From<char> for Request {
    fn from(item: char) -> Request {
        match item {
            'm' => Request::ConstructMainView,
            'd' => Request::ConstructDiffView,
            'l' => Request::ConstructLogView,
            'p' => Request::ConstructPagerView,
            't' => Request::ConstructTreeView,
            'f' => Request::ConstructBlobView,
            'g' => Request::ConstructGrepView,
            'b' => Request::ConstructBlameView,
            'r' => Request::ConstructRefsView,
            'y' => Request::ConstructStashView,
            'h' => Request::ConstructHelpView,
            's' => Request::ConstructStatusView,
            'c' => Request::ConstructStageView,
            _ => Request::InvalidRequest,
        }
    }
}

impl From<KeyCode> for Request {
    fn from(item: KeyCode) -> Request {
        match item {
            KeyCode::Char('m') => Request::ConstructMainView,
            KeyCode::Char('d') => Request::ConstructDiffView,
            KeyCode::Char('l') => Request::ConstructLogView,
            KeyCode::Char('p') => Request::ConstructPagerView,
            KeyCode::Char('t') => Request::ConstructTreeView,
            KeyCode::Char('f') => Request::ConstructBlobView,
            KeyCode::Char('g') => Request::ConstructGrepView,
            KeyCode::Char('b') => Request::ConstructBlameView,
            KeyCode::Char('r') => Request::ConstructRefsView,
            KeyCode::Char('y') => Request::ConstructStashView,
            KeyCode::Char('h') => Request::ConstructHelpView,
            KeyCode::Char('s') => Request::ConstructStatusView,
            KeyCode::Char('c') => Request::ConstructStageView,
            _ => Request::InvalidRequest,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::request::Request;
    use rstest::rstest;

    use crossterm::event::KeyCode;

    #[rstest]
    #[case('m', Request::ConstructMainView)]
    #[case('d', Request::ConstructDiffView)]
    #[case('l', Request::ConstructLogView)]
    #[case('p', Request::ConstructPagerView)]
    #[case('t', Request::ConstructTreeView)]
    #[case('f', Request::ConstructBlobView)]
    #[case('g', Request::ConstructGrepView)]
    #[case('b', Request::ConstructBlameView)]
    #[case('r', Request::ConstructRefsView)]
    #[case('y', Request::ConstructStashView)]
    #[case('h', Request::ConstructHelpView)]
    #[case('s', Request::ConstructStatusView)]
    #[case('c', Request::ConstructStageView)]
    #[case('0', Request::InvalidRequest)]
    fn test_request_from_char(#[case] c: char, #[case] expected: Request) {
        assert_eq!(Request::from(c), expected);
    }

    #[rstest]
    #[case(KeyCode::Char('m'), Request::ConstructMainView)]
    #[case(KeyCode::Char('d'), Request::ConstructDiffView)]
    #[case(KeyCode::Char('l'), Request::ConstructLogView)]
    #[case(KeyCode::Char('p'), Request::ConstructPagerView)]
    #[case(KeyCode::Char('t'), Request::ConstructTreeView)]
    #[case(KeyCode::Char('f'), Request::ConstructBlobView)]
    #[case(KeyCode::Char('g'), Request::ConstructGrepView)]
    #[case(KeyCode::Char('b'), Request::ConstructBlameView)]
    #[case(KeyCode::Char('r'), Request::ConstructRefsView)]
    #[case(KeyCode::Char('y'), Request::ConstructStashView)]
    #[case(KeyCode::Char('h'), Request::ConstructHelpView)]
    #[case(KeyCode::Char('s'), Request::ConstructStatusView)]
    #[case(KeyCode::Char('c'), Request::ConstructStageView)]
    #[case(KeyCode::Char('0'), Request::InvalidRequest)]
    fn test_request_from_crossterm_event_key_code(#[case] c: KeyCode, #[case] expected: Request) {
        assert_eq!(Request::from(c), expected);
    }
}
