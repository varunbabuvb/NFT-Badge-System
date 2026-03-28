# NFT Badge System

### Project Description
The **NFT Badge System** is a decentralized application (dApp) built on the Soroban smart contract platform. It serves as a digital credentialing engine that allows authorized entities to issue immutable, verifiable badges as non-fungible tokens. Each badge contains the recipient's identifier, the achievement name, and a blockchain-verified timestamp, ensuring that credentials cannot be forged or tampered with.

### Project Vision
Our vision is to democratize professional and academic recognition by moving away from centralized, siloed credential databases. By using the Stellar network, we aim to provide a low-cost, high-speed infrastructure where individuals truly own their achievements and can prove them instantly to any third party globally.

### Key Features
* **Immutable Provenance:** Every badge is hard-coded into the ledger with a permanent ID and issuance date.
* **Permissionless Verification:** Any user can verify the validity of a badge by calling the `view_badge` function without requiring admin permission.
* **Resource Optimized:** Leverages Soroban's efficient state storage to minimize "ledger footprint" and transaction costs.
* **Transparent Accounting:** A public counter (`total_badges`) tracks the growth of the ecosystem in real-time.

### Future Scope
* **Soulbound Integration:** Restricting badge transfers to prevent "credential selling" and ensure the badge stays with the earner.
* **Metadata URIs:** Adding support for links to IPFS-hosted images or detailed achievement criteria.
* **Expiry Logic:** Implementing auto-expiring badges for certifications that require periodic renewal (e.g., CPR or Safety training).
* **Role-Based Access (RBAC):** Adding advanced admin controls to allow multiple authorized "issuers" while maintaining a single source of truth.
* ## contract details:
* ## contract ID:CBJRDPVG5NVKFLTVT5TJCPCUG2TIFGCPZY3EK42XK7QIYIKNJBCNRWSI
* <img width="1790" height="642" alt="image" src="https://github.com/user-attachments/assets/7a53b194-d351-47d0-9569-eaec89d4bdda" />
