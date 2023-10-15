use anyhow::Result;

pub trait Repository<T, PK, UPD> {
    fn select_all(&mut self) -> Result<Vec<T>> {
        todo!()
    }

    fn select_by_id(&mut self, id: PK) -> Result<Option<T>> {
        todo!()
    }

    fn insert(&mut self, row: T) -> Result<UPD> {
        todo!()
    }

    fn update_by_id(&mut self, id: PK) -> Result<UPD> {
        todo!()
    }

    fn delete_by_id(&mut self, id: PK) -> Result<UPD> {
        todo!()
    }
}

