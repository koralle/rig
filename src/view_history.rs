use crate::request::Request;

#[derive(Debug, Eq, PartialEq)]
enum ViewHistoryItem {
    Main,
    Diff,
    Log,
    Pager,
    Tree,
    Blob,
    Grep,
    Blame,
    Refs,
    Stash,
    Help,
    Status,
    Stage,
}

impl From<ViewHistoryItem> for Request {
    fn from(item: ViewHistoryItem) -> Request {
        match item {
            ViewHistoryItem::Main => Request::ConstructMainView,
            ViewHistoryItem::Diff => Request::ConstructDiffView,
            ViewHistoryItem::Log => Request::ConstructLogView,
            ViewHistoryItem::Pager => Request::ConstructPagerView,
            ViewHistoryItem::Tree => Request::ConstructTreeView,
            ViewHistoryItem::Blob => Request::ConstructBlobView,
            ViewHistoryItem::Grep => Request::ConstructGrepView,
            ViewHistoryItem::Blame => Request::ConstructBlameView,
            ViewHistoryItem::Refs => Request::ConstructRefsView,
            ViewHistoryItem::Stash => Request::ConstructStashView,
            ViewHistoryItem::Help => Request::ConstructHelpView,
            ViewHistoryItem::Status => Request::ConstructStatusView,
            ViewHistoryItem::Stage => Request::ConstructStageView,
        }
    }
}

struct ViewHistory(Vec<ViewHistoryItem>);

impl ViewHistory {
    fn new() -> Self {
        ViewHistory(Vec::new())
    }

    fn push(&mut self, item: ViewHistoryItem) {
        self.0.push(item);
    }

    fn pop(&mut self) -> Option<ViewHistoryItem> {
        self.0.pop()
    }

    fn is_empty(self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::request::Request;
    use rstest::rstest;

    use crate::view_history::ViewHistoryItem;

    #[rstest]
    #[case(ViewHistoryItem::Main, Request::ConstructMainView)]
    #[case(ViewHistoryItem::Diff, Request::ConstructDiffView)]
    #[case(ViewHistoryItem::Log, Request::ConstructLogView)]
    #[case(ViewHistoryItem::Pager, Request::ConstructPagerView)]
    #[case(ViewHistoryItem::Tree, Request::ConstructTreeView)]
    #[case(ViewHistoryItem::Blob, Request::ConstructBlobView)]
    #[case(ViewHistoryItem::Grep, Request::ConstructGrepView)]
    #[case(ViewHistoryItem::Blame, Request::ConstructBlameView)]
    #[case(ViewHistoryItem::Refs, Request::ConstructRefsView)]
    #[case(ViewHistoryItem::Stash, Request::ConstructStashView)]
    #[case(ViewHistoryItem::Help, Request::ConstructHelpView)]
    #[case(ViewHistoryItem::Status, Request::ConstructStatusView)]
    #[case(ViewHistoryItem::Stage, Request::ConstructStageView)]
    fn test_request_from_view_history_item(#[case] c: ViewHistoryItem, #[case] expected: Request) {
        assert_eq!(Request::from(c), expected);
    }
}
