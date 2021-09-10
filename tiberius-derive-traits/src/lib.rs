pub trait FromRow<'a> {
    fn from_row(__row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

pub trait FromRowCopy {
    fn from_row(__row: &tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}

pub trait FromRowOwned {
    fn from_row(__row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: std::marker::Sized;
}
