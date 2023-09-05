#[allow(unused)]
pub struct Item {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) completed: bool,
    pub(crate) deleted: bool,
    pub(crate) created_at: Option<i64>,
    pub(crate) completed_at: Option<i64>,
    pub(crate) deleted_at: Option<i64>,
}

#[allow(unused)]
impl Item {
    /// Associated Functions,
    /// which first parameter is NOT `&self(self: &Self)`,
    /// `&mut self(self: &mut Self)` or `self(self: Self)`
    pub fn new(
        id: u32,
        name: &str,
        completed: bool,
        deleted: bool,
        created_at: Option<i64>,
        completed_at: Option<i64>,
        deleted_at: Option<i64>,
    ) -> Self {
        todo!("implement new function");
    }

    /// methods,
    /// which first parameter is `&self(self: &Self)`,
    /// `&mut self(self: &mut Self)` or `self(self: Self)`
    /// if `id` property is not `pub`
    /// we can use methods to support `getter` or `setter`
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_prettier_string(&self) -> String {
        todo!();
    }
}
