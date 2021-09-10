pub trait FromRow<'a> {
    fn from_row(row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

pub trait FromRowCopy {
    fn from_row(row: &tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

pub trait FromRowOwned {
    fn from_row(row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}
