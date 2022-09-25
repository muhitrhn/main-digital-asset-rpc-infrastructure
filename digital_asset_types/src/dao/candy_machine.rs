//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "candy_machine"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub features: Option<u64>,
    pub authority: Vec<u8>,
    pub mint_authority: Option<u8>,
    pub wallet: Vec<u8>,
    pub token_mint: Option<Vec<u8>>,
    pub items_redeemed: i32,
    pub candy_guard_pda: Option<Vec<u8>>,
    pub version: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Features,
    Authority,
    MintAuthority,
    Wallet,
    TokenMint,
    ItemsRedeemed,
    CandyGuardPda,
    Version,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Vec<u8>;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CandyMachineData,
    CandyGuardGroup,
    CandyMachineHiddenSettings,
    CandyMachineEndSettings,
    CandyMachineGatekeeper,
    CandyMachineWhitelistMintSettings,
    CandyMachineCreators,
    CandyMachineConfigLineSettings,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Binary.def(),
            Self::Features => ColumnType::Integer.def().null(),
            Self::Authority => ColumnType::Binary.def(),
            Self::MintAuthority => ColumnType::Binary.def().null(),
            Self::Wallet => ColumnType::Binary.def(),
            Self::TokenMint => ColumnType::Binary.def().null(),
            Self::ItemsRedeemed => ColumnType::Integer.def(),
            Self::CandyGuardPda => ColumnType::Binary.def().null(),
            Self::Version => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CandyMachineHiddenSettings => {
                Entity::has_one(super::candy_machine_hidden_settings::Entity).into()
            }
            Self::CandyMachineEndSettings => {
                Entity::has_one(super::candy_machine_end_settings::Entity).into()
            }
            Self::CandyMachineGatekeeper => {
                Entity::has_one(super::candy_machine_gatekeeper::Entity).into()
            }
            Self::CandyMachineWhitelistMintSettings => {
                Entity::has_one(super::candy_machine_whitelist_mint_settings::Entity).into()
            }
            Self::CandyMachineCreators => {
                Entity::has_many(super::candy_machine_creators::Entity).into()
            }
            Self::CandyMachineConfigLineSettings => {
                Entity::has_one(super::candy_machine_config_line_settings::Entity).into()
            }
            Self::CandyMachineData => Entity::has_one(super::candy_machine_data::Entity).into(),
            Self::CandyGuardGroup => Entity::belongs_to(super::candy_guard_group::Entity)
                .from(Column::MintAuthority)
                .to(super::candy_guard_group::Column::CandyMachineId)
                .into(),
            // TODO ^^ should this be switched, pk on guard group is id(mint_authority) and fk on cm is mint_authority
        }
    }
}

impl Related<super::candy_machine_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineData.def()
    }
}

impl Related<super::candy_guard_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyGuardGroup.def()
    }
}

impl Related<super::candy_machine_whitelist_mint_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineWhitelistMintSettings.def()
    }
}

impl Related<super::candy_machine_gatekeeper::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineGatekeeper.def()
    }
}

impl Related<super::candy_machine_end_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineEndSettings.def()
    }
}

impl Related<super::candy_machine_hidden_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineHiddenSettings.def()
    }
}

impl Related<super::candy_machine_creators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineCreators.def()
    }
}

impl Related<super::candy_machine_config_line_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineConfigLineSettings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
