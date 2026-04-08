//! L1 genesis configurations.
use crate::alloc::string::ToString;
use alloc::{collections::BTreeMap, string::String};
use alloy_eips::eip7840::BlobParams;
use alloy_genesis::EthashConfig;
use core::{fmt::Display, ops::Deref};
use kona_genesis::L1ChainConfig;

use alloy_chains::NamedChain;
use alloy_primitives::{Address, U256, address, map::HashMap};

/// L1 chain configuration.
/// Simple wrapper around the [`L1ChainConfig`] type from the `alloy-genesis` crate.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct L1Config(L1ChainConfig);

impl Deref for L1Config {
    type Target = L1ChainConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<L1ChainConfig> for L1Config {
    fn from(spec: L1ChainConfig) -> Self {
        Self(spec)
    }
}

impl From<L1Config> for L1ChainConfig {
    fn from(val: L1Config) -> Self {
        val.0
    }
}

impl L1Config {
    const MAINNET_TTD: u128 = 58_750_000_000_000_000_000_000u128;
    const MAINNET_DEPOSIT_CONTRACT_ADDRESS: Address =
        address!("0x00000000219ab540356cbb839cbe05303d7705fa");

    const SEPOLIA_TTD: u128 = 17_000_000_000_000_000u128;
    const SEPOLIA_DEPOSIT_CONTRACT_ADDRESS: Address =
        address!("0x7f02c3e3c98b133055b8b348b2ac625669ed295d");
    const SEPOLIA_MERGE_NETSPLIT_BLOCK: u64 = 1735371;

    const HOLESKY_TTD: u128 = 0;
    const HOLESKY_DEPOSIT_CONTRACT_ADDRESS: Address =
        address!("0x4242424242424242424242424242424242424242");

    /// Get the genesis for a given chain ID.
    pub fn get_l1_genesis(chain_id: u64) -> Result<Self, L1GenesisGetterErrors> {
        match NamedChain::try_from(chain_id)
            .map_err(|_| L1GenesisGetterErrors::ChainIDDoesNotExist(chain_id))?
        {
            NamedChain::Mainnet => Ok(Self::mainnet()),
            NamedChain::Sepolia => Ok(Self::sepolia()),
            NamedChain::Holesky => Ok(Self::holesky()),
            _ => Err(L1GenesisGetterErrors::UnknownChainID(chain_id)),
        }
    }

    fn default_blob_schedule() -> BTreeMap<String, BlobParams> {
        BTreeMap::from([
            (
                alloy_hardforks::EthereumHardfork::Cancun.name().to_string().to_lowercase(),
                BlobParams::cancun(),
            ),
            (
                alloy_hardforks::EthereumHardfork::Prague.name().to_string().to_lowercase(),
                BlobParams::prague(),
            ),
            (
                alloy_hardforks::EthereumHardfork::Osaka.name().to_string().to_lowercase(),
                BlobParams::osaka(),
            ),
            (
                alloy_hardforks::EthereumHardfork::Bpo1.name().to_string().to_lowercase(),
                BlobParams::bpo1(),
            ),
            (
                alloy_hardforks::EthereumHardfork::Bpo2.name().to_string().to_lowercase(),
                BlobParams::bpo2(),
            ),
        ])
    }

    /// Parse the mainnet genesis.
    pub fn mainnet() -> Self {
        let mut cfg = L1ChainConfig::default();
        cfg.chain_id = NamedChain::Mainnet.into();
        cfg.homestead_block =
            alloy_hardforks::EthereumHardfork::Homestead.mainnet_activation_block();
        cfg.dao_fork_block = alloy_hardforks::EthereumHardfork::Dao.mainnet_activation_block();
        cfg.dao_fork_support = true;
        cfg.eip150_block =
            alloy_hardforks::EthereumHardfork::Tangerine.mainnet_activation_block();
        cfg.eip155_block =
            alloy_hardforks::EthereumHardfork::SpuriousDragon.mainnet_activation_block();
        cfg.eip158_block =
            alloy_hardforks::EthereumHardfork::SpuriousDragon.mainnet_activation_block();
        cfg.byzantium_block =
            alloy_hardforks::EthereumHardfork::Byzantium.mainnet_activation_block();
        cfg.constantinople_block =
            alloy_hardforks::EthereumHardfork::Constantinople.mainnet_activation_block();
        cfg.petersburg_block =
            alloy_hardforks::EthereumHardfork::Petersburg.mainnet_activation_block();
        cfg.istanbul_block =
            alloy_hardforks::EthereumHardfork::Istanbul.mainnet_activation_block();
        cfg.muir_glacier_block =
            alloy_hardforks::EthereumHardfork::MuirGlacier.mainnet_activation_block();
        cfg.berlin_block = alloy_hardforks::EthereumHardfork::Berlin.mainnet_activation_block();
        cfg.london_block = alloy_hardforks::EthereumHardfork::London.mainnet_activation_block();
        cfg.arrow_glacier_block =
            alloy_hardforks::EthereumHardfork::ArrowGlacier.mainnet_activation_block();
        cfg.gray_glacier_block =
            alloy_hardforks::EthereumHardfork::GrayGlacier.mainnet_activation_block();
        cfg.shanghai_time =
            alloy_hardforks::EthereumHardfork::Shanghai.mainnet_activation_timestamp();
        cfg.cancun_time =
            alloy_hardforks::EthereumHardfork::Cancun.mainnet_activation_timestamp();
        cfg.prague_time =
            alloy_hardforks::EthereumHardfork::Prague.mainnet_activation_timestamp();
        cfg.osaka_time = alloy_hardforks::EthereumHardfork::Osaka.mainnet_activation_timestamp();
        cfg.bpo1_time = alloy_hardforks::EthereumHardfork::Bpo1.mainnet_activation_timestamp();
        cfg.bpo2_time = alloy_hardforks::EthereumHardfork::Bpo2.mainnet_activation_timestamp();
        cfg.bpo3_time = alloy_hardforks::EthereumHardfork::Bpo3.mainnet_activation_timestamp();
        cfg.bpo4_time = alloy_hardforks::EthereumHardfork::Bpo4.mainnet_activation_timestamp();
        cfg.bpo5_time = alloy_hardforks::EthereumHardfork::Bpo5.mainnet_activation_timestamp();
        cfg.ethash = Some(EthashConfig {});
        cfg.blob_schedule = Self::default_blob_schedule();
        cfg.terminal_total_difficulty = Some(U256::from(Self::MAINNET_TTD));
        cfg.deposit_contract_address = Some(Self::MAINNET_DEPOSIT_CONTRACT_ADDRESS);
        Self(cfg)
    }

    /// Parse the sepolia genesis.
    pub fn sepolia() -> Self {
        let mut cfg = L1ChainConfig::default();
        cfg.chain_id = NamedChain::Sepolia.into();
        cfg.homestead_block =
            alloy_hardforks::EthereumHardfork::Homestead.sepolia_activation_block();
        cfg.dao_fork_block = alloy_hardforks::EthereumHardfork::Dao.sepolia_activation_block();
        cfg.dao_fork_support = true;
        cfg.eip150_block =
            alloy_hardforks::EthereumHardfork::Tangerine.sepolia_activation_block();
        cfg.eip155_block =
            alloy_hardforks::EthereumHardfork::SpuriousDragon.sepolia_activation_block();
        cfg.eip158_block =
            alloy_hardforks::EthereumHardfork::Byzantium.sepolia_activation_block();
        cfg.byzantium_block =
            alloy_hardforks::EthereumHardfork::Byzantium.sepolia_activation_block();
        cfg.constantinople_block =
            alloy_hardforks::EthereumHardfork::Constantinople.sepolia_activation_block();
        cfg.petersburg_block =
            alloy_hardforks::EthereumHardfork::Petersburg.sepolia_activation_block();
        cfg.istanbul_block =
            alloy_hardforks::EthereumHardfork::Istanbul.sepolia_activation_block();
        cfg.muir_glacier_block =
            alloy_hardforks::EthereumHardfork::MuirGlacier.sepolia_activation_block();
        cfg.berlin_block = alloy_hardforks::EthereumHardfork::Berlin.sepolia_activation_block();
        cfg.london_block = alloy_hardforks::EthereumHardfork::London.sepolia_activation_block();
        cfg.arrow_glacier_block =
            alloy_hardforks::EthereumHardfork::ArrowGlacier.sepolia_activation_block();
        cfg.gray_glacier_block =
            alloy_hardforks::EthereumHardfork::GrayGlacier.sepolia_activation_block();
        cfg.shanghai_time =
            alloy_hardforks::EthereumHardfork::Shanghai.sepolia_activation_timestamp();
        cfg.cancun_time =
            alloy_hardforks::EthereumHardfork::Cancun.sepolia_activation_timestamp();
        cfg.prague_time =
            alloy_hardforks::EthereumHardfork::Prague.sepolia_activation_timestamp();
        cfg.osaka_time = alloy_hardforks::EthereumHardfork::Osaka.sepolia_activation_timestamp();
        cfg.bpo1_time = alloy_hardforks::EthereumHardfork::Bpo1.sepolia_activation_timestamp();
        cfg.bpo2_time = alloy_hardforks::EthereumHardfork::Bpo2.sepolia_activation_timestamp();
        cfg.bpo3_time = alloy_hardforks::EthereumHardfork::Bpo3.sepolia_activation_timestamp();
        cfg.bpo4_time = alloy_hardforks::EthereumHardfork::Bpo4.sepolia_activation_timestamp();
        cfg.bpo5_time = alloy_hardforks::EthereumHardfork::Bpo5.sepolia_activation_timestamp();
        cfg.ethash = Some(EthashConfig {});
        cfg.blob_schedule = Self::default_blob_schedule();
        cfg.terminal_total_difficulty = Some(U256::from(Self::SEPOLIA_TTD));
        cfg.merge_netsplit_block = Some(Self::SEPOLIA_MERGE_NETSPLIT_BLOCK);
        cfg.deposit_contract_address = Some(Self::SEPOLIA_DEPOSIT_CONTRACT_ADDRESS);
        Self(cfg)
    }

    /// Parse the holesky genesis.
    pub fn holesky() -> Self {
        let mut cfg = L1ChainConfig::default();
        cfg.chain_id = NamedChain::Holesky.into();
        cfg.homestead_block = Some(0);
        cfg.dao_fork_block = Some(0);
        cfg.dao_fork_support = true;
        cfg.eip150_block = Some(0);
        cfg.eip155_block = Some(0);
        cfg.eip158_block = Some(0);
        cfg.byzantium_block = Some(0);
        cfg.constantinople_block = Some(0);
        cfg.petersburg_block = Some(0);
        cfg.istanbul_block = Some(0);
        cfg.muir_glacier_block = Some(0);
        cfg.berlin_block = Some(0);
        cfg.london_block = Some(0);
        cfg.arrow_glacier_block = Some(0);
        cfg.gray_glacier_block = Some(0);
        cfg.shanghai_time = Some(0);
        cfg.cancun_time =
            alloy_hardforks::EthereumHardfork::Cancun.holesky_activation_timestamp();
        cfg.prague_time =
            alloy_hardforks::EthereumHardfork::Prague.holesky_activation_timestamp();
        cfg.osaka_time = alloy_hardforks::EthereumHardfork::Osaka.holesky_activation_timestamp();
        cfg.bpo1_time = alloy_hardforks::EthereumHardfork::Bpo1.holesky_activation_timestamp();
        cfg.bpo2_time = alloy_hardforks::EthereumHardfork::Bpo2.holesky_activation_timestamp();
        cfg.bpo3_time = alloy_hardforks::EthereumHardfork::Bpo3.holesky_activation_timestamp();
        cfg.bpo4_time = alloy_hardforks::EthereumHardfork::Bpo4.holesky_activation_timestamp();
        cfg.bpo5_time = alloy_hardforks::EthereumHardfork::Bpo5.holesky_activation_timestamp();
        cfg.ethash = Some(EthashConfig {});
        cfg.blob_schedule = Self::default_blob_schedule();
        cfg.terminal_total_difficulty = Some(U256::from(Self::HOLESKY_TTD));
        cfg.deposit_contract_address = Some(Self::HOLESKY_DEPOSIT_CONTRACT_ADDRESS);
        Self(cfg)
    }

    /// Build the l1 chain configurations from the genesis dump files.
    pub fn build_l1_configs() -> HashMap<u64, L1ChainConfig> {
        let mut l1_configs = HashMap::default();
        l1_configs.insert(NamedChain::Mainnet.into(), Self::mainnet().0);
        l1_configs.insert(NamedChain::Sepolia.into(), Self::sepolia().0);
        l1_configs.insert(NamedChain::Holesky.into(), Self::holesky().0);
        l1_configs
    }
}

/// Errors that can occur when trying to get the l1 genesis config for a given chain ID.
#[derive(Debug)]
pub enum L1GenesisGetterErrors {
    /// The chain ID does not exist in the [`NamedChain`] registry.
    ChainIDDoesNotExist(u64),
    /// The chain ID is unknown.
    UnknownChainID(u64),
    /// Failed to parse the genesis.
    ParseGenesisError(serde_json::Error),
}

impl Display for L1GenesisGetterErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<serde_json::Error> for L1GenesisGetterErrors {
    fn from(error: serde_json::Error) -> Self {
        Self::ParseGenesisError(error)
    }
}

#[cfg(test)]
mod tests {
    use alloy_hardforks::EthereumHardfork;

    use super::*;

    #[test]
    fn test_get_l1_genesis() {
        let l1_config = L1Config::get_l1_genesis(NamedChain::Mainnet.into()).unwrap();
        assert_eq!(l1_config.chain_id, u64::from(NamedChain::Mainnet));

        let l1_config = L1Config::get_l1_genesis(NamedChain::Sepolia.into()).unwrap();
        assert_eq!(l1_config.chain_id, u64::from(NamedChain::Sepolia));

        let l1_config = L1Config::get_l1_genesis(NamedChain::Holesky.into()).unwrap();
        assert_eq!(l1_config.chain_id, u64::from(NamedChain::Holesky));

        let l1_config = L1Config::get_l1_genesis(1000000).unwrap_err();
        assert!(matches!(l1_config, L1GenesisGetterErrors::ChainIDDoesNotExist(1000000)));
    }

    #[test]
    fn test_get_l1_bpo_mainnet() {
        /// BPO1 hardfork activation timestamp
        const MAINNET_BPO1_TIMESTAMP: u64 = 1_765_290_071;

        /// BPO2 hardfork activation timestamp
        const MAINNET_BPO2_TIMESTAMP: u64 = 1_767_747_671;

        let mainnet = L1Config::mainnet();

        assert_eq!(mainnet.blob_schedule.len(), 5);
        assert_eq!(
            mainnet.blob_schedule.get(&EthereumHardfork::Bpo1.name().to_lowercase()).unwrap(),
            &BlobParams::bpo1()
        );
        assert_eq!(
            mainnet.blob_schedule.get(&EthereumHardfork::Bpo2.name().to_lowercase()).unwrap(),
            &BlobParams::bpo2()
        );

        let blob_schedule = mainnet.blob_schedule_blob_params();
        assert_eq!(blob_schedule.scheduled.len(), 2);
        assert_eq!(blob_schedule.scheduled[0].0, MAINNET_BPO1_TIMESTAMP);
        assert_eq!(blob_schedule.scheduled[1].0, MAINNET_BPO2_TIMESTAMP);
        assert_eq!(blob_schedule.scheduled[0].1, BlobParams::bpo1());
        assert_eq!(blob_schedule.scheduled[1].1, BlobParams::bpo2());
    }

    #[test]
    fn test_get_l1_bpo_sepolia() {
        /// BPO1 hardfork activation timestamp
        const SEPOLIA_BPO1_TIMESTAMP: u64 = 1761017184;

        /// BPO2 hardfork activation timestamp
        const SEPOLIA_BPO2_TIMESTAMP: u64 = 1761607008;

        let sepolia = L1Config::sepolia();

        assert_eq!(sepolia.blob_schedule.len(), 5);
        assert_eq!(
            sepolia.blob_schedule.get(&EthereumHardfork::Bpo1.name().to_lowercase()).unwrap(),
            &BlobParams::bpo1()
        );
        assert_eq!(
            sepolia.blob_schedule.get(&EthereumHardfork::Bpo2.name().to_lowercase()).unwrap(),
            &BlobParams::bpo2()
        );

        let blob_schedule = sepolia.blob_schedule_blob_params();
        assert_eq!(blob_schedule.scheduled.len(), 2);
        assert_eq!(blob_schedule.scheduled[0].0, SEPOLIA_BPO1_TIMESTAMP);
        assert_eq!(blob_schedule.scheduled[1].0, SEPOLIA_BPO2_TIMESTAMP);
        assert_eq!(blob_schedule.scheduled[0].1, BlobParams::bpo1());
        assert_eq!(blob_schedule.scheduled[1].1, BlobParams::bpo2());
    }

    #[test]
    fn test_get_l1_bpo_holesky() {
        /// BPO1 hardfork activation timestamp
        const HOLESKY_BPO1_TIMESTAMP: u64 = 1759800000;

        /// BPO2 hardfork activation timestamp
        const HOLESKY_BPO2_TIMESTAMP: u64 = 1760389824;

        let holesky = L1Config::holesky();

        assert_eq!(holesky.blob_schedule.len(), 5);
        assert_eq!(
            holesky.blob_schedule.get(&EthereumHardfork::Bpo1.name().to_lowercase()).unwrap(),
            &BlobParams::bpo1()
        );
        assert_eq!(
            holesky.blob_schedule.get(&EthereumHardfork::Bpo2.name().to_lowercase()).unwrap(),
            &BlobParams::bpo2()
        );

        let blob_schedule = holesky.blob_schedule_blob_params();
        assert_eq!(blob_schedule.scheduled.len(), 2);
        assert_eq!(blob_schedule.scheduled[0].0, HOLESKY_BPO1_TIMESTAMP);
        assert_eq!(blob_schedule.scheduled[1].0, HOLESKY_BPO2_TIMESTAMP);
        assert_eq!(blob_schedule.scheduled[0].1, BlobParams::bpo1());
        assert_eq!(blob_schedule.scheduled[1].1, BlobParams::bpo2());
    }
}
