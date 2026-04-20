🎲 Thesis Advisor Gacha DApp

Transparent & Tamper-Proof Blockchain-Based Thesis Advisor Allocation System

📖 Project Description

Gacha Dosen DApp is a smart contract application built on the Stellar blockchain network using the Soroban SDK.

Its simple function: To randomly and fairly allocate thesis advisors to students.

Thanks to blockchain technology, the random number generation system in this app is 100% pure and cannot be manipulated (set up by the university or admins). Every advisor allocation result will also be permanently recorded on the Stellar network, allowing anyone to view it transparently. No more drama or suspicion in advisor allocations!

🎯 Project Vision

The goal of this project is to improve the campus administration system by:

100% Fair: Eliminating favoritism or advisor "booking" because everything is drawn purely by the system.

Transparent: All students and campus staff can openly view the available advisor pool and the draw history.

Permanent & Tamper-Proof (Immutable): Once a gacha result is out, it cannot be edited, deleted, or altered by anyone.

Campus Modernization: Bringing future technology (Web3) into the campus academic system.

✨ Key Features

1. Dynamic Advisor Pool

Admins can add lecturers into the "draw pool".

Stored data includes the lecturer's name and area of expertise (e.g., Web Development, UI/UX Design).

2. Provably Fair Gacha System

The draw process is driven by Soroban's secure built-in Pseudo-Random Number Generator (PRNG).

Only requires a single execution to match a student with an available advisor.

The system will automatically reject the draw if the advisor data is empty, preventing errors.

3. Undeletable History

A digital notebook (Ledger) that stores gacha results in real-time.

Easy to find out which student is advised by a specific lecturer.

4. Powered by Stellar Network

Runs on the Stellar blockchain, known for being extremely fast and cheap.

Ready to be used on a large scale (e.g., for all students in a faculty).

🔗 Smart Contract Details

Network: Stellar Testnet

Contract Address (ID): CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ

🚀 Future Development Plan (Roadmap)

Phase 1: UI/UX (Short-Term)

Modern Web: Build a website interface using Laravel and Tailwind CSS to eliminate the need for black terminal typing.

Gacha Animation: Add Loot Box or slot machine style animations (designed with Figma) to make the drawing experience feel like playing a game.

Quota Limit: Add a maximum student limit for each advisor to prevent overload.

Phase 2: Advanced Features (Medium-Term)

Expertise Filter: Students can choose to "gacha" specifically within a certain category (e.g., only rolling for Computer Network specialist advisors).

Admin Access Control: Lock the add/remove advisor feature so only campus admin digital wallets can perform these actions.

Phase 3: Large Scale (Long-Term)

Cross-Department Scaling: Develop the system to be used by multiple study programs simultaneously.

Automated Reporting: A system that can export gacha results from the blockchain directly into official campus Excel/PDF report formats.

🛠️ Technical Requirements

Soroban SDK

Rust Programming Language

Stellar CLI & Freighter Wallet

💻 How to Run the App

You can try interacting with this app directly via the terminal (Soroban CLI) on the Stellar Testnet. Follow these steps:

1. Add an Advisor to the Draw Pool:

stellar contract invoke --id CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ --source-account default --network testnet -- add_dosen --name Pak_Budi --expertise Spesialis_Web_Dev


2. Let's Gacha Your Advisor!
Replace "Zackhary" with your name or your friend's name:

stellar contract invoke --id CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ --source-account default --network testnet -- roll_gacha --student_name Zackhary


3. View Draw Result History:

stellar contract invoke --id CAXCONIMPWPG4SP5DUO7Q2LSUXP6LUZ2AMNXW46O3MB5SA7QNUYLRBDZ --source-account default --network testnet -- get_history
