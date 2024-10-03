use cosmos_sdk_proto::cosmos::{bank::v1beta1::Metadata, base::v1beta1::Coin as ProtoCoin};
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg};
use prost::Message;

use crate::msg::{MsgBurn, MsgCreateDenom, MsgMint, MsgSetDenomMetadata};

pub fn tf_create_denom_msg<T>(sender: impl Into<String>, denom: impl Into<String>) -> CosmosMsg<T>
where
    T: CustomMsg,
{
    let create_denom_msg = MsgCreateDenom {
        sender: sender.into(),
        subdenom: denom.into(),
    };

    CosmosMsg::Stargate {
        type_url: MsgCreateDenom::TYPE_URL.to_string(),
        value: Binary::from(create_denom_msg.encode_to_vec()),
    }
}

pub fn tf_mint_msg<T>(
    sender: impl Into<String>,
    coin: Coin,
    receiver: impl Into<String>,
) -> Vec<CosmosMsg<T>>
where
    T: CustomMsg,
{
    let sender_addr: String = sender.into();
    let receiver_addr: String = receiver.into();

    let mint_msg = MsgMint {
        sender: sender_addr.clone(),
        amount: Some(ProtoCoin {
            denom: coin.denom.to_string(),
            amount: coin.amount.to_string(),
        }),
        mint_to_address: receiver_addr.clone(),
    };

    return vec![CosmosMsg::Stargate {
        type_url: MsgMint::TYPE_URL.to_string(),
        value: Binary::from(mint_msg.encode_to_vec()),
    }];
}

pub fn tf_burn_msg<T>(sender: impl Into<String>, coin: Coin) -> CosmosMsg<T>
where
    T: CustomMsg,
{
    let burn_msg = MsgBurn {
        sender: sender.into(),
        amount: Some(ProtoCoin {
            denom: coin.denom,
            amount: coin.amount.to_string(),
        }),
        burn_from_address: "".to_string(),
    };

    CosmosMsg::Stargate {
        type_url: MsgBurn::TYPE_URL.to_string(),
        value: Binary::from(burn_msg.encode_to_vec()),
    }
}

pub fn tf_set_denom_metadata_msg<T>(sender: impl Into<String>, metadata: Metadata) -> CosmosMsg<T>
where
    T: CustomMsg,
{
    let set_denom_metadata_msg = MsgSetDenomMetadata {
        sender: sender.into(),
        metadata: Some(metadata),
    };

    CosmosMsg::Stargate {
        type_url: MsgSetDenomMetadata::TYPE_URL.to_string(),
        value: Binary::from(set_denom_metadata_msg.encode_to_vec()),
    }
}
