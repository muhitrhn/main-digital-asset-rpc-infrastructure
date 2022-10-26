//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "candy_guard_group"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub label: Option<String>,
    pub candy_guard_id: Vec<u8>,
    pub bot_tax_lamports: Option<i64>,
    pub bot_tax_last_instruction: Option<bool>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub third_party_signer_key: Option<Vec<u8>>,
    pub nft_payment_destination: Option<Vec<u8>>,
    pub nft_payment_required_collection: Option<Vec<u8>>,
    pub mint_limit_id: Option<i16>,
    pub mint_limit_limit: Option<i16>,
    pub gatekeeper_network: Option<Vec<u8>>,
    pub gatekeeper_expire_on_use: Option<bool>,
    pub sol_payment_lamports: Option<i64>,
    pub sol_payment_destination: Option<Vec<u8>>,
    pub redeemed_amount_maximum: Option<i64>,
    pub address_gate_address: Option<Vec<u8>>,
    pub freeze_sol_payment_lamports: Option<i64>,
    pub freeze_sol_payment_destination: Option<Vec<u8>>,
    pub token_gate_amount: Option<i64>,
    pub token_gate_mint: Option<Vec<u8>>,
    pub nft_gate_required_collection: Option<Vec<u8>>,
    pub token_burn_amount: Option<i64>,
    pub token_burn_mint: Option<Vec<u8>>,
    pub nft_burn_required_collection: Option<Vec<u8>>,
    pub token_payment_amount: Option<i64>,
    pub token_payment_mint: Option<Vec<u8>>,
    pub token_payment_destination_ata: Option<Vec<u8>>,
    pub allow_list_merkle_root: Option<Vec<u8>>,
    pub freeze_token_payment_amount: Option<i64>,
    pub freeze_token_payment_mint: Option<Vec<u8>>,
    pub freeze_token_payment_destination_ata: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Label,
    CandyGuardId,
    BotTaxLamports,
    BotTaxLastInstruction,
    StartDate,
    EndDate,
    ThirdPartySignerKey,
    NftPaymentDestination,
    NftPaymentRequiredCollection,
    MintLimitId,
    MintLimitLimit,
    GatekeeperNetwork,
    GatekeeperExpireOnUse,
    SolPaymentLamports,
    SolPaymentDestination,
    RedeemedAmountMaximum,
    AddressGateAddress,
    FreezeSolPaymentLamports,
    FreezeSolPaymentDestination,
    TokenGateAmount,
    TokenGateMint,
    NftGateRequiredCollection,
    TokenBurnAmount,
    TokenBurnMint,
    NftBurnRequiredCollection,
    TokenPaymentAmount,
    TokenPaymentMint,
    TokenPaymentDestinationAta,
    AllowListMerkleRoot,
    FreezeTokenPaymentAmount,
    FreezeTokenPaymentMint,
    FreezeTokenPaymentDestinationAtaAta,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CandyGuard,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Label => ColumnType::String(Some(50u32)).def().null(),
            Self::CandyGuardId => ColumnType::Binary.def(),
            Self::BotTaxLamports => ColumnType::BigInteger.def().null(),
            Self::BotTaxLastInstruction => ColumnType::Boolean.def().null(),
            Self::StartDate => ColumnType::BigInteger.def().null(),
            Self::EndDate => ColumnType::BigInteger.def().null(),
            Self::ThirdPartySignerKey => ColumnType::Binary.def().null(),
            Self::NftPaymentDestination => ColumnType::Binary.def().null(),
            Self::NftPaymentRequiredCollection => ColumnType::Binary.def().null(),
            Self::MintLimitId => ColumnType::SmallInteger.def().null(),
            Self::MintLimitLimit => ColumnType::SmallInteger.def().null(),
            Self::GatekeeperNetwork => ColumnType::Binary.def().null(),
            Self::GatekeeperExpireOnUse => ColumnType::Boolean.def().null(),
            Self::SolPaymentLamports => ColumnType::BigInteger.def().null(),
            Self::SolPaymentDestination => ColumnType::Binary.def().null(),
            Self::RedeemedAmountMaximum => ColumnType::BigInteger.def().null(),
            Self::AddressGateAddress => ColumnType::Binary.def().null(),
            Self::FreezeSolPaymentLamports => ColumnType::BigInteger.def().null(),
            Self::FreezeSolPaymentDestination => ColumnType::Binary.def().null(),
            Self::TokenGateAmount => ColumnType::BigInteger.def().null(),
            Self::TokenGateMint => ColumnType::Binary.def().null(),
            Self::NftGateRequiredCollection => ColumnType::Binary.def().null(),
            Self::TokenBurnAmount => ColumnType::BigInteger.def().null(),
            Self::TokenBurnMint => ColumnType::Binary.def().null(),
            Self::NftBurnRequiredCollection => ColumnType::Binary.def().null(),
            Self::TokenPaymentAmount => ColumnType::BigInteger.def().null(),
            Self::TokenPaymentMint => ColumnType::Binary.def().null(),
            Self::TokenPaymentDestinationAta => ColumnType::Binary.def().null(),
            Self::AllowListMerkleRoot => ColumnType::Binary.def().null(),
            Self::FreezeTokenPaymentAmount => ColumnType::BigInteger.def().null(),
            Self::FreezeTokenPaymentMint => ColumnType::Binary.def().null(),
            Self::FreezeTokenPaymentDestinationAtaAta => ColumnType::Binary.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CandyGuard => Entity::belongs_to(super::candy_guard::Entity)
                .from(Column::CandyGuardId)
                .to(super::candy_guard::Column::Id)
                .into(),
        }
    }
}

impl Related<super::candy_guard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyGuard.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
