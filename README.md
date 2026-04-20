Gacha Dosen Pembimbing DApp

Gacha Dosen DApp - Blockchain-Based Decentralized Thesis Advisor Allocation System

Project Description

Gacha Dosen DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and immutable platform for randomly assigning thesis or final project advisors (Dosen Pembimbing) to students.

By leveraging the blockchain's deterministic Pseudo-Random Number Generator (PRNG), this contract ensures that the allocation process is 100% fair, verifiable, and free from human bias or manual intervention. Each allocation is permanently recorded on the Stellar network.

Project Vision

Our vision is to revolutionize the academic allocation process by:

Ensuring Absolute Fairness: Eliminating bias by using on-chain random generation for advisor assignments.

Guaranteeing Transparency: Allowing all students and academic staff to publicly verify the gacha pool and assignment history.

Providing Immutability: Creating a permanent, tamper-proof record of advisor allocations that cannot be altered once the gacha is rolled.

Modernizing Academic Systems: Bridging the gap between traditional campus administration and decentralized Web3 technologies.

Key Features

1. Dynamic Advisor Pool

Admin functionality to securely add lecturers to the gacha pool.

Store detailed profiles including the lecturer's name and specific field of expertise (e.g., Full-Stack, UI/UX Design).

2. Provably Fair Gacha System

Core allocation engine powered by Soroban's secure PRNG.

One-click function to randomly match a student with an available advisor.

Prevents execution if the pool is empty to ensure system stability.

3. Immutable History Tracking

Real-time, transparent ledger of all gacha results.

Easy data retrieval mapping each student to their assigned advisor.

4. Stellar Network Integration

Built on the highly efficient and low-cost Stellar network.

Scalable architecture ready to handle entire faculty cohorts.

Contract Details

Network: Stellar Testnet

Contract Address: CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ

Future Scope

Short-Term Enhancements

Modern Web Interface: Building a sleek, responsive frontend dashboard using Tailwind CSS and Laravel to visualize the gacha results.

Enhanced UI/UX: Designing interactive, game-like "loot box" animations and intuitive user flows using Figma before frontend implementation.

Quota System: Implementing a maximum capacity limit for each lecturer so they don't get assigned too many students.

Medium-Term Development

Expertise Filtering: Allowing students to roll the gacha specifically within a desired field (e.g., rolling only for lecturers with Informatics Engineering backgrounds).

Admin Access Control: Implementing authorization so only verified campus administration wallets can add or remove lecturers from the pool.

Frontend DApp Integration: Connecting the smart contract to the browser using Freighter Wallet for seamless interaction without the terminal.

Long-Term Vision

Cross-Department Scaling: Expanding the contract to support multiple study programs simultaneously.

DAO Governance: Allowing the student body (like MPM/BEM) to audit and propose changes to the allocation algorithm.

Automated Reporting: Exporting the blockchain history directly into official campus academic administration formats.

Technical Requirements

Soroban SDK

Rust Programming Language

Stellar CLI & Freighter Wallet

Getting Started

Interact with the contract on the Stellar Testnet using the Soroban CLI.

1. Populate the Pool (Add Dosen):

stellar contract invoke --id CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ --source-account default --network testnet -- add_dosen --name Pak_Budi --expertise Spesialis_Web_Dev


2. Roll the Gacha!

stellar contract invoke --id CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ --source-account default --network testnet -- roll_gacha --student_name Zackhary


(Try simulating it for your peers as well by changing the student name to Agung, Falih, or Shahid).

3. View Assignment History:

stellar contract invoke --id CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ --source-account default --network testnet -- get_history


Gacha Dosen DApp - Fair, Transparent, and Immutable Academic Allocations.