#![allow(clippy::expect_fun_call)] // My calls are so fun

use super::data_structures::ForeignKeyConstraint;
use super::inference::get_primary_keys;
use super::table_data::TableName;
use crate::database::InferConnection;

pub fn remove_unsafe_foreign_keys_for_codegen(
    conn: &mut InferConnection,
    foreign_keys: &[ForeignKeyConstraint],
    safe_tables: &[TableName],
) -> Vec<ForeignKeyConstraint> {
    let duplicates = foreign_keys
        .iter()
        .map(ForeignKeyConstraint::ordered_tables)
        .filter(|tables| {
            let dup_count = foreign_keys
                .iter()
                .filter(|fk| tables == &fk.ordered_tables())
                .count();
            dup_count > 1
        })
        .collect::<Vec<_>>();

    foreign_keys
        .iter()
        .filter(|fk| fk.parent_table != fk.child_table)
        .filter(|fk| safe_tables.contains(&fk.parent_table))
        .filter(|fk| safe_tables.contains(&fk.child_table))
        .filter(|fk| fk.foreign_key_columns.len() == 1)
        .filter(|fk| {
            let pk_columns = get_primary_keys(conn, &fk.parent_table).expect(&format!(
                "Error loading primary keys for `{}`",
                fk.parent_table
            ));
            pk_columns.len() == 1 && Some(&pk_columns[0]) == fk.primary_key_columns.first()
        })
        .filter(|fk| !duplicates.contains(&fk.ordered_tables()))
        .cloned()
        .collect()
}
