#[derive(Debug, PartialEq, Eq)]
pub enum SparseSetErrorHandle {
    EntryAttachmentFail,
    EntryRemovalFail,
    InvalidEntry,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ECSErrorHandle {
    ComponentAttachmentFail,
    ComponentRemovalFail,
    EntityFailure,
    InvalidEntity
}
