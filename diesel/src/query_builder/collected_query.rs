use super::{
    AstPass, BindCollector, MovableBindCollector, QueryFragment, QueryId, SendableCollector,
};
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::result::QueryResult;

pub struct CollectedQuery<'a, DB>
where
    DB: Backend,
    <DB as Backend>::BindCollector<'a>: MovableBindCollector<DB>,
{
    sql: String,
    safe_to_cache_prepared: bool,
    movable_bind_collector: <DB::BindCollector<'a> as MovableBindCollector<DB>>::MovableData,
}

impl<'a, DB> CollectedQuery<'a, DB>
where
    DB: Backend,
    // for<'a> <DB as Backend>::BindCollector<'a>: MovableBindCollector<DB, MovableData = T>,
    <DB as Backend>::BindCollector<'a>: MovableBindCollector<DB>,
{
    pub fn new(
        sql: String,
        safe_to_cache_prepared: bool,
        movable_bind_collector: <DB::BindCollector<'a> as MovableBindCollector<DB>>::MovableData,
    ) -> Self {
        Self {
            sql,
            safe_to_cache_prepared,
            movable_bind_collector,
        }
    }

    pub fn dbg(&self) {
        println!(
            "Mo coll {} {}",
            &self.sql,
            self.movable_bind_collector.describe()
        );
    }
}

impl<'a, DB> QueryFragment<DB> for CollectedQuery<'a, DB>
where
    DB: Backend,
    // T: SendableCollector,
    // for<'a> <DB as Backend>::BindCollector<'a>: MovableBindCollector<DB, MovableData = T>,
    for<'bind> <DB as Backend>::BindCollector<'bind>: MovableBindCollector<DB>,
    // for<'a> T: MovableBindCollector<'a, DB>,
    // for<'a> DB: Backend<BindCollector<'a> as IntoBinds<'a, DB>::OwnedBuffer = T>,
    // binds: Vec<<<DB as Backend>::BindCollector<'param> as IntoBinds<'param, DB>>::OwnedBuffer>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        if !self.safe_to_cache_prepared {
            pass.unsafe_to_cache_prepared();
        }
        pass.push_sql(&self.sql);
        // pass.push_bind_collector_data(&self.movable_bind_collector)
        Ok(())
    }
}

impl<'a, DB> QueryId for CollectedQuery<'a, DB>
where
    DB: Backend,
    <DB as Backend>::BindCollector<'a>: MovableBindCollector<DB>,
{
    type QueryId = ();

    const HAS_STATIC_QUERY_ID: bool = false;
}
