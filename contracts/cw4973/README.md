# Cw4973 - Account-bound Tokens (ABT)

Cw4973 is implemented based on the description of [EIP-4973](https://eips.ethereum.org/EIPS/eip-4973). Some requirements in the description will be changed for suitable with CosmWasm characteristics.

## Maintainers

This contract source code is provided and supported by [Aura project](https://aura.network/).

## Contributing

If you are working on an Cw4973 NFT project as well and wish to give input, please raise issues and/or PRs. Additional maintainers can be added if they show commitment to the project.

## Specification
Cw4973 is extended from Cw721-base with some modifications in execution section:
* All the `metadata` structures are unchanged.
* All the `query functions` and `query messages` are unchanged.
* All the `execute functions` and `execute messages` are removed.
* We add 3 following `execute functions` and `execute messages` to the contract:
    - `Give`
    - `Take`
    - `Unequip`

### `Give`
Creates and transfers the ownership of an ABT from the transaction's `info.sender` to address `to`. Throws unless the `signature` represents a Signature created by [Interface OfflineAminoSigner](https://cosmos.github.io/cosmjs/latest/amino/interfaces/OfflineAminoSigner.html#signAmino) of the message data structure from [Interface StdSignDoc](https://cosmos.github.io/cosmjs/latest/amino/interfaces/StdSignDoc.html): `Agreement(address active,address passive,string tokenURI)` expressing address `to`'s explicit agreement to be publicly associated with `info.sender` and string `tokenURI`. A unique `token_id` must be generated by type-casting the `Vec<u8>` hash value of Interface StdSignDoc structured data to `hex`. A successful execution will emit the event:
```json
{
    "action": "mint",
    "minter": info.sender,
    "owner": to,
    "token_id": token_id
}
```
Once an ABT exists as an hex `token_id` in the contract, function `give(...)` must throw.

The paramters of function include:
* `to`: The receiver of the ABT.
* `uri`: A distinct Uniform Resource Identifier (URI) for a given ABT.
* `signature`: A signature created by [Interface OfflineAminoSigner](https://cosmos.github.io/cosmjs/latest/amino/interfaces/OfflineAminoSigner.html#signAmino) on the message data structure from [Interface StdSignDoc](https://cosmos.github.io/cosmjs/latest/amino/interfaces/StdSignDoc.html) signed by address `to`.

Return A unique hex `token_id` generated by type-casting the `Vec<u8>` hash value of Interface StdSignDoc structured data.

### `Take`
Creates and transfers the ownership of an ABT from an address `from` to the transaction's `info.sender`. Throws unless the `signature` represents a Signature created by [Interface OfflineAminoSigner](https://cosmos.github.io/cosmjs/latest/amino/interfaces/OfflineAminoSigner.html#signAmino) of the message data structure from [Interface StdSignDoc](https://cosmos.github.io/cosmjs/latest/amino/interfaces/StdSignDoc.html): `Agreement(address active,address passive,string tokenURI)` expressing address `from`'s explicit agreement to be publicly associated with `info.sender` and string `tokenURI`. A unique `token_id` must be generated by type-casting the `Vec<u8>` hash value of Interface StdSignDoc structured data to `hex`. A successful execution will emit the event:
```json
{
    "action": "mint",
    "minter": info.sender,
    "owner": info.sender,
    "token_id": token_id
}
```
Once an ABT exists as an hex `token_id` in the contract, function `take(...)` must throw.

The paramters of function include:
* `from`: The origin of the ABT.
* `uri`: A distinct Uniform Resource Identifier (URI) for a given ABT.
* `signature`: A signature created by [Interface OfflineAminoSigner](https://cosmos.github.io/cosmjs/latest/amino/interfaces/OfflineAminoSigner.html#signAmino) on the message data structure from [Interface StdSignDoc](https://cosmos.github.io/cosmjs/latest/amino/interfaces/StdSignDoc.html) signed by address `from`.

Return A unique hex `token_id` generated by type-casting the `Vec<u8>` hash value of Interface StdSignDoc structured data.

### `Unequip`
Removes the hex `token_id` from an account. A event will be emitted:
```json
{
    "action": "unequip",
    "token_id": token_id,
    "owner": info.sender
}

```
The paramters of function include:
* `token_id`: The identifier for an ABT.
## The examples
In this section, we will provide the examples of the signable message, the signature and execute messages being used in the contract. The language is used in the examples is javascript on the NodeJS.
### The signable message
To create a signable message, use the following code:
```javascript
const amino = require('@cosmjs/amino');

function createMessageToSign(chainID, active, passive, uri) {
    const AGREEMENT = 'Agreement(address active,address passive,string tokenURI)';

    // create message to sign based on concating AGREEMENT, signer, receiver, and uri
    const message = AGREEMENT + active + passive + uri;

    const mess = {
        type: "sign/MsgSignData",
        value: {
            signer: String(passive),
            data: String(message)
        }
    };

    const fee = {
        gas: "0",
        amount: []
    };

    const messageToSign = amino.makeSignDoc(mess, fee, String(chainID), "",  0, 0);

    return messageToSign;
}
```
The parameters of function include:
* `chainID`: The ID of chain (`String`). This parameter is used to ensure that the signature cannot be used on the other chains or other environments.
* `active`: The address of `info.sender` (`String`).
* `passive`: The address of signer (`String`).
* `uri`: A distinct Uniform Resource Identifier (URI) for a given ABT (`String`).

The returned message should be like this:
```json
{
  chain_id: "aura-testnet",
  account_number: "0",
  sequence: "0",
  fee: { gas: "0", amount: [] },
  msgs: {
    type: "sign/MsgSignData",
    value: {
      signer: "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n",
      data: "Agreement(address active,address passive,string tokenURI)aura1fqj2redmssckrdeekhkcvd2kzp9f4nks4fctrtaura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0nhttps://yellow-bizarre-puma-439.mypinata.cloud/ipfs/QmcCTHB3UFak5RY4qedSbiR7Raj1odPWsU1pTyddtxfSxH/8555"
    }
  },
  memo: ""
}
```

### The signature
To create the signature of a signable message, use the following code:
1. Create a `config.js`:
```javascript
'use strict';

const Testnet = {
    rpcEndpoint: 'https://rpc.dev.aura.network',
    prefix: 'aura',
    denom: 'utaura',
    chainId: 'aura-testnet',
    broadcastTimeoutMs: 5000,
    broadcastPollIntervalMs: 1000
};

let defaultChain = Testnet;

defaultChain.deployer_mnemonic = process.env.MNEMONIC;

module.exports = {
    Testnet
};
```
2. This example is creating a signature for `Take` function is below. In the function, the address `from` is `admin` of contract and he will sign on the message to allow the user as `infor.sender` take a nft:
```javascript
const chainConfig = require('./chain').defaultChain;
const amino = require('@cosmjs/amino');

async function getPermitSignatureAmino(messageToSign) {
    const deployerWallet = await amino.Secp256k1HdWallet.fromMnemonic(
        chainConfig.deployer_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    // const adminAccount = deployerWallet.getAccounts()[0];
    const adminAccount = (await deployerWallet.getAccounts())[0];

    // signed message
    const signedDoc = await deployerWallet.signAmino(adminAccount.address, messageToSign);

    // pubkey must be encoded in base64
    let permitSignature = {
        "hrp": "aura",
        "pub_key": Buffer.from(adminAccount.pubkey).toString('base64'),
        "signature": signedDoc.signature.signature,
    }

    return permitSignature;
}
```

The signature should be like this:
```json
{
    hrp: "aura",
    pub_key: "A9EkWupSnnFmIIEWG7WtMc0Af/9oEuEeSRTKF/bJrCfh",
    signature: "s3cAqMjAFazchg09Ji+2Mzw+uAvS7LoN+znboociSdMyLM58C4H4a9A38v+68i8+fhTg3bXbP1NnrlwduLdXCA=="
}
```
Note that, we must provide the `hrp` value is used on your chain in the signature because this value will be used to verify the signature.

### Execute messages
1. `Give` message
The `give` message should be like this:
```json
{
    give: {
        to: "aura1fqj2redmssckrdeekhkcvd2kzp9f4nks4fctrt",
        uri: "https://yellow-bizarre-puma-439.mypinata.cloud/ipfs/QmcCTHB3UFak5RY4qedSbiR7Raj1odPWsU1pTyddtxfSxH/8555",
        signature: {
            hrp: "aura",
            pub_key: "A7Ek6upSnnFmIIEWG7WtMc0Af/9oEuEe12345/bJr890",
            signature: "hahaqMjAFazchg09Ji+2Mzw+uAvS7LoN+znabcdiSdMyLM58C4H4a9A38v+68i8+fhTg3bXbP1NnrlwduLdXYZ=="
        }
    }
}
```

2. `Take` message
The `take` message should be like this:
```json
{
    take: {
        from: "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n",
        uri: "https://yellow-bizarre-puma-439.mypinata.cloud/ipfs/QmcCTHB3UFak5RY4qedSbiR7Raj1odPWsU1pTyddtxfSxH/8555",
        signature: {
            hrp: "aura",
            pub_key: "A9EkWupSnnFmIIEWG7WtMc0Af/9oEuEeSRTKF/bJrCfh",
            signature: "s3cAqMjAFazchg09Ji+2Mzw+uAvS7LoN+znboociSdMyLM58C4H4a9A38v+68i8+fhTg3bXbP1NnrlwduLdXCA=="
        }
    }
}
```

3. `Unequip` message
The `unequip` message should be like this:
```json
{
    unequip: {
        token_id: "d52d068e9a04fa4e3ca38707d427cb6f2ba9335fe74b072a84e496c220b87225"
    }
}
```