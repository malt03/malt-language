#[derive(Debug, PartialEq)]
pub(crate) struct LocalValue<'a> {
    pub(crate) name: &'a str,
}
