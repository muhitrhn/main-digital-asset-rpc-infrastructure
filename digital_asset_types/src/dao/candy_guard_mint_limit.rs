//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "candy_guard_mint_limit"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub limit: u16,
    pub candy_machine_id: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Limit,
    CandyMachineId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Vec<u8>;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CandyGuardGroup,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Limit => ColumnType::Integer.def(),
            Self::CandyMachineId => ColumnType::Binary.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CandyGuardGroup => Entity::belongs_to(super::candy_guard_group::Entity)
                .from(Column::CandyMachineId)
                .to(super::candy_guard_group::Column::CandyMachineId)
                .into(),
        }
    }
}

impl Related<super::candy_guard_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyGuardGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
