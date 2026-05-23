# Indonesian Biodiversity Registry

**Indonesian Biodiversity Registry** - Blockchain-Based Decentralized Tropical Species Record & Auditing System

## Project Description

Indonesian Biodiversity Registry is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It serves as a secure, immutable, and plug-and-play backend upgrade for the traditional Web2 biodiversity indexing platform (**biodiversitas-indonesia.or.id**). The contract ensures that environmental observation records, conservation statuses, and habitat logs are stored transparently on-chain, eliminating reliance on centralized database providers and protecting crucial ecological research logs from unauthorized manipulation.

The system allows authorized researchers and institutions to create, read, update, and delete species records, leveraging the high efficiency, data integrity, and security of the Stellar network. Each record is uniquely identified and stored within the contract's instance storage using native Soroban host-managed collections.

## Project Vision

Our vision is to revolutionize ecological preservation and data integrity in the digital age by:

- **Decentralizing Ecological Data**: Moving vital biodiversity records from vulnerable centralized Web2 hosting servers to a global, distributed Stellar blockchain network.
- **Ensuring Data Sovereignty**: Empowering environmental researchers, NGOs, and field biologists to have complete and verifiable ownership over their discovery logs.
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of vulnerable and endangered tropical species that cannot be altered, forged, or deleted by single points of failure.
- **Enhancing Transparency**: Leveraging public ledger history to verify all conservation status updates, creating a high-trust framework for global ecological audits.
- **Building Trustless Registries**: Creating an ecosystem where the historical preservation of flora and fauna data is guaranteed by immutable smart contract logic, not by company promises.

We envision a future where digital conservation records are universally trusted and sovereign, empowering global communities with completely autonomous and verifiable climate action tracking assets.

## Key Features

### 1. **On-Chain Species Entry Creation (Create)**

- Record new scientific discoveries with a single smart contract transaction call.
- Support for complex text-based attributes using dynamic `soroban_sdk::String` types.
- Input data constraints mapped directly to standard Web2 fields (`nama_ilmiah`, `nama_lokal`, `status_iucn`, `lokasi_ditemukan`).
- Automated validation checks using native `panic!()` triggers to completely block duplicate entry IDs from corrupting the ledger state.

### 2. **Efficient Global Data Retrieval (Read)**

- Fetch the entire live biodiversity sequence array in a single execution call via the SDK Vector (`Vec<BiodiversityRecord>`).
- Seamless read-only backend API state extraction optimized for direct rendering on frontend web maps and data sheets.
- Instant fallback rendering that initializes an empty vector if no records exist, avoiding unhandled null data exceptions.
- Real-time synchronization directly bound to the global Stellar ledger states.

### 3. **Dynamic Observation Modifications (Update)**

- Modify conservation status tiers (e.g., updating an IUCN Red List classification) or recent habitat locations on-chain.
- Target items securely via unique numeric item IDs, altering string records in active state storage dynamically.
- Returns explicit Boolean feedback states (`true` on success, `false` on missing targets) for precise user confirmation interfaces.

### 4. **Secure Error Correction and Deletion (Delete)**

- Permanently purge invalid or faulty record entries from the active sequence by supplying their unique numerical database ID.
- Frees contract state storage capacity by automatically shifting vector index lines sequentially, optimizing memory and gas.
- Immediate on-chain structural update across validators right after transaction completion.

### 5. **Modern Stellar Architecture Integration**

- Capitalizes on ultra-low transaction costs and sub-second block validation speeds of Stellar's Soroban engine.
- Built using advanced Rust compilation modules and Soroban SDK host-allocated environments.
- Scalable database schema designed specifically to host massive quantities of tropical ecosystem records.
- Completely interoperable with future on-chain decentralized carbon credits or financial asset contracts.

## Contract Details

- Network: Stellar Testnet
- Contract ID: CBVLMC25OABBKSGFNZOEZECWWEECDJZJLU3L2QGRKBDRPNY25JHCESX5

## Future Scope

### Short-Term Enhancements

1. **Researcher Identity Access Control (ACL)**: Implement administrative signature checks (`msg.sender` validation) ensuring only verified institutions can edit rows.
2. **Geographical Coordinates Compression**: Support for parsing exact latitudinal and longitudinal habitat pin values alongside text descriptions.
3. **Multi-Language Scientific Strings**: Expand text serialization mapping to support localized dialect terms for remote vernacular names.
4. **On-Chain Search Engine**: Build decentralized sorting filters directly inside the contract methods to return elements matched by status keywords.

### Medium-Term Development

5. **Multi-Signature Verification DAO**: Establish a community governance system where newly proposed species entries require multi-sig sign-offs from multiple verified biologists before being appended to the blockchain.
   - Consensus-based validation for rare species observations
   - Dynamic permission level allocations for junior/senior field researchers
   - Cryptographic timestamping for peer-reviewed field studies
6. **Automated Eco-Philanthropy Fundraising Tracker**: Direct attachment hooks that allow individual Web3 addresses to fund specific endangered species records natively using XLM tokens.
7. **Species Tokenization Layer**: Capability to auto-generate unique eco-certificates or non-fungible badges representing tokenized backing for a specific species' habitat zone.
8. **Decentralized Storage (IPFS) Integration**: Store heavy attachments (such as high-resolution wildlife photos, GPS track files, and DNA sequencing strings) on IPFS while saving their immutable cryptographic hashes within this Soroban contract structure.

### Long-Term Vision

9. **Cross-Chain Environmental Reporting Sync**: Mirror critical endangered lists seamlessly across multiple layer-1 smart contract ecosystems to increase public visibility.
10. **Fully Decentralized Frontend Platform**: Host the entire user dashboard of the upgraded platform on permanent decentralized networks like IPFS or Arweave.
11. **Oraclized Ecological Feeds**: Partner with automated IoT environmental sensors to auto-update location logs on-chain via decentralized data feed oracles.
12. **Zero-Knowledge Privacy Triggers**: Implement ZK-proof cryptography to mask the ultra-precise coordinates of highly targeted endangered species from poachers, while proving their existence to verified auditors.
13. **DAO-Driven Climate Allocation**: Establish a full token-weighted ecosystem deciding where accumulated environmental grant funding is deployed across registered habitats.
14. **Decentralized ID (DID) Integration**: Connect institutional wallets to verified Web3 profile systems for transparent credentials checking.

### Enterprise Features

15. **Corporate ESG Compliance Hub**: Package the immutable ledger data for corporate entities needing transparent environmental audit compliance material.
16. **Time-Locked Habitat Protection Logs**: Immutable time-locks verifying exactly how long specific conservation land sectors have remained strictly protected without changes.
17. **Automated Regulatory Reporting Triggers**: Programmatically emit event topics directly to government compliance agencies upon rapid changes in regional IUCN statuses.
18. **International Multi-Agency Registry Federation**: Standardize data parameters across various cross-border wildlife federations using consistent on-chain schemas.

---

## Technical Requirements

- Soroban SDK (v0.20+ recommended)
- Rust Programming Language (Stable Toolchain)
- Stellar Blockchain Testnet Environment
- Freighter Browser Wallet Extension

## Getting Started

Deploy this smart contract backend via Soroban Studio to Stellar's Testnet network and interact with it using these core endpoints:

- `add_record(env, id, nama_ilmiah, nama_lokal, status_iucn, lokasi_ditemukan)` - Creates a new biodiversity registry entry on-chain.
- `get_all_records(env)` - Pulls all active, structurally mapped tropical species logs from the contract instance.
- `update_record(env, id, new_status, new_lokasi)` - Alters the conservation attributes of a specified ID.
- `delete_record(env, id)` - Removes a corrupted entry from the current on-chain monitoring array sequence.

---

**Indonesian Biodiversity Registry** - Securing Ecosystem Transparency on the Blockchain