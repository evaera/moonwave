use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Realm {
    Client,
    Server,
    Plugin,
}
