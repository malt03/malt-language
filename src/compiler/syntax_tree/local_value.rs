#[derive(Debug, PartialEq, Clone)]
pub(crate) struct LocalValue<'a> {
    pub(crate) name: &'a str,
    pub(crate) type_: &'a str,
}
