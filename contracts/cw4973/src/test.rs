#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, DepsMut, };

use crate::{Cw4973Contract, ExecuteMsg, PermitSignature, entry, ContractError};
use cw721_base::msg::{QueryMsg, InstantiateMsg as Cw721InstantiateMsg};
use cw721::ContractInfoResponse;

const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

const MINTER_ADDRESS: &str = "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n";
const MINTER_PUBKEY: &str = "A/TyvFSR0UDXmfN6EWGVqMClEaSWSTWwVEzhbl8SSfi+";
const TESTER_ADDRESS: &str = "aura1fqj2redmssckrdeekhkcvd2kzp9f4nks4fctrt";
const TESTER_PUBKEY: &str = "A9EkWupSnnFmIIEWG7WtMc0Af/9oEuEeSRTKF/bJrCfh";


const CHAIN_ID: &str = "euphoria-1";
const CHAIN_ID_FAKE: &str = "euphoria-2";

const URI: &str = "https://yellow-bizarre-puma-439.mypinata.cloud/ipfs/QmcCTHB3UFak5RY4qedSbiR7Raj1odPWsU1pTyddtxfSxH/8555";
// const URI_FAKE: &str = "https://yellow-bizarre-puma-439.mypinata.cloud/ipfs/QmcCTHB3UFak5RY4qedSbiR7Raj1odPWsU1pTyddtxfSxH/8557";

const SIGNATURE_TAKE: &str = "s3cAqMjAFazchg09Ji+2Mzw+uAvS7LoN+znboociSdMyLM58C4H4a9A38v+68i8+fhTg3bXbP1NnrlwduLdXCA==";
const SIGNATURE_TAKE_FAKE: &str = "a3cAqMjAFazchg09Ji+2Mzw+uAvS7LoN+znboociSdMyLM58C4H4a9A38v+68i8+fhTg3bXbP1NnrlwduLdXCA==";

const SIGNATURE_GIVE: &str = "yTkGJViQsCRkclfKzN5Akff4DijnZTBrCLZwZ63DTPNAGan2FfQwpEtpb23YXsNU+aJTZazD6Iij4v0idH43cQ==";
const SIGNATURE_GIVE_FAKE: &str = "zTkGJViQsCRkclfKzN5Akff4DijnZTBrCLZwZ63DTPNAGan2FfQwpEtpb23YXsNU+aJTZazD6Iij4v0idH43cQ==";

// function to change value of mock values
fn my_mock_env(chain_id: &str) -> cosmwasm_std::Env {
    // change values for testing
    let mut env = mock_env();
    env.block.chain_id = chain_id.to_string();
    env
}


fn setup_contract<'a>(deps: DepsMut<'_>) -> Cw4973Contract<'a> {
    let contract = Cw4973Contract::default();
    let msg = Cw721InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER_ADDRESS),
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();

    let contract = setup_contract(deps.as_mut());

    // it worked, let's query the state
    let res = contract.query(deps.as_ref(), mock_env(), QueryMsg::ContractInfo {}).unwrap();
    let value: ContractInfoResponse = from_binary(&res).unwrap();
    assert_eq!(value.name, CONTRACT_NAME);
    assert_eq!(value.symbol, SYMBOL);
}

#[test]
fn cannot_take_nft_because_not_from_minter() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: TESTER_PUBKEY.to_string(),
        signature: SIGNATURE_TAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let take_msg = ExecuteMsg::Take {
        from: TESTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, take_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::Unauthorized)));
}

#[test]
fn cannot_take_nft_when_change_chain_id() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID_FAKE);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: TESTER_PUBKEY.to_string(),
        signature: SIGNATURE_TAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let take_msg = ExecuteMsg::Take {
        from: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(TESTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, take_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::InvalidSignature)));
}


#[test]
fn cannot_take_nft_from_himself() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: TESTER_PUBKEY.to_string(),
        signature: SIGNATURE_TAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let take_msg = ExecuteMsg::Take {
        from: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, take_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::CannotTakeFromSelf)));
}

#[test]
fn cannot_take_nft_because_signature_invalid() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: TESTER_PUBKEY.to_string(),
        signature: SIGNATURE_TAKE_FAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let take_msg = ExecuteMsg::Take {
        from: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(TESTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, take_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::InvalidSignature)));
}

#[test]
fn cannot_take_nft_because_hrp_incorrect() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura111".to_string(),
        pub_key: TESTER_PUBKEY.to_string(),
        signature: SIGNATURE_TAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let take_msg = ExecuteMsg::Take {
        from: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(TESTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, take_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::InvalidSigner)));
}

// take a nft
#[test]
fn take_nft() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: TESTER_PUBKEY.to_string(),
        signature: SIGNATURE_TAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let take_msg = ExecuteMsg::Take {
        from: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(TESTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, take_msg).unwrap();

    assert_eq!(0, res.messages.len());
}

#[test]
fn cannot_give_nft_because_sender_not_minter() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: MINTER_PUBKEY.to_string(),
        signature: SIGNATURE_GIVE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let give_msg = ExecuteMsg::Give {
        to: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(TESTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, give_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::Unauthorized)));
}

#[test]
fn cannot_give_nft_when_change_chain_id() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: MINTER_PUBKEY.to_string(),
        signature: SIGNATURE_GIVE_FAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let give_msg = ExecuteMsg::Give {
        to: TESTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, give_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::InvalidSignature)));
}

#[test]
fn cannot_give_nft_for_himself() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: MINTER_PUBKEY.to_string(),
        signature: SIGNATURE_GIVE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let give_msg = ExecuteMsg::Give {
        to: MINTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, give_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::CannotGiveToSelf)));
}

#[test]
fn cannot_give_nft_because_signature_invalid() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura111".to_string(),
        pub_key: MINTER_PUBKEY.to_string(),
        signature: SIGNATURE_GIVE_FAKE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let give_msg = ExecuteMsg::Give {
        to: TESTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, give_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::InvalidSignature)));
}

#[test]
fn cannot_give_nft_because_hrp_incorrect() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura111".to_string(),
        pub_key: MINTER_PUBKEY.to_string(),
        signature: SIGNATURE_GIVE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let give_msg = ExecuteMsg::Give {
        to: TESTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, give_msg);

    // check if error is returned\
    assert!(matches!(res, Err(ContractError::InvalidSigner)));
}

// give a nft
#[test]
fn give_nft() {
    // get mock dependencies
    let mut deps = mock_dependencies();

    // change chanin id of mock env
    let env = my_mock_env(CHAIN_ID);

    // setup contract
    let _contract: Cw4973Contract = setup_contract(deps.as_mut());

    // create permitSignature
    let permit_signature = PermitSignature {
        hrp: "aura".to_string(),
        pub_key: MINTER_PUBKEY.to_string(),
        signature: SIGNATURE_GIVE.to_string(),
    };

    // prepare take msg from minter address, uri and signature
    let give_msg = ExecuteMsg::Give {
        to: TESTER_ADDRESS.to_string(),
        uri: URI.to_string(),
        signature: permit_signature
    };

    // call take function
    let info = mock_info(MINTER_ADDRESS, &[]);
    let res = entry::execute(deps.as_mut(), env, info, give_msg).unwrap();

    assert_eq!(0, res.messages.len());
}

