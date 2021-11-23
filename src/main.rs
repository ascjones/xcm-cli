#[subxt::subxt(runtime_metadata_path = "relay-chain.scale")]
pub mod relay_chain {}

use relay_chain::runtime_types::xcm::{
    VersionedMultiAssets,
    VersionedMultiLocation,
    v0::{
        junction::{
            Junction,
            NetworkId,
        },
        multi_location::MultiLocation as V0MultiLocation,
    },
    v1::{
        multiasset::{
            AssetId,
            MultiAsset,
            MultiAssets,
            Fungibility,
        },
        multilocation::{
            Junctions,
            MultiLocation as V1MultiLocation
        },
    }
};

use color_eyre::eyre::WrapErr;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build()
        .await
        .context("Error connecting to substrate node")?
        .to_runtime_api::<relay_chain::RuntimeApi<relay_chain::DefaultConfig>>();

    // todo: check and pass args
    let dest_parachain = 1002;
    let dest_account = AccountKeyring::Bob.to_account_id().into();
    let amount = 1_000_000_000_000;

    let dest = VersionedMultiLocation::V0(
        V0MultiLocation::X1(Junction::Parachain(dest_parachain))
    );
    let beneficiary = VersionedMultiLocation::V0(
        V0MultiLocation::X1(Junction::AccountId32 { network: NetworkId::Any, id: dest_account })
    );
    let assets = VersionedMultiAssets::V1(
        MultiAssets(
            vec![
                MultiAsset {
                    id: AssetId::Concrete(V1MultiLocation { parents: 0, interior: Junctions::Here }),
                    fun: Fungibility::Fungible(amount)
                }
            ]
        )
    );
    let fee_asset_item = 0;

    let result = api
        .tx()
        .xcm_pallet()
        .teleport_assets(dest, beneficiary, assets, fee_asset_item)
        .sign_and_submit_then_watch(&signer)
        .await
        .context("Error submitting extrinsic")?;

    for event in result.events.iter() {
        println!("{:?}", event)
    }

    Ok(())
}