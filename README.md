# Campus Board DApp

**Campus Board DApp** - Blockchain-Based Student Report & Task Management System

---

## Project Description

Campus Board DApp is a decentralized smart contract application built on the Stellar blockchain using Soroban SDK. The platform provides a transparent and immutable system for university students to create, manage, and track campus-related reports and academic tasks directly on-chain.

This decentralized application enables students to report campus facility issues, submit academic reminders, and monitor the progress of reports through blockchain technology.

The system allows users to:
- Create new campus reports or tasks
- Retrieve all submitted reports
- Update report status
- Delete reports when necessary

Each report is uniquely identified and permanently managed through predefined smart contract functions on the Stellar blockchain.

---

## Project Vision

Our vision is to modernize campus communication and student reporting systems through decentralized technology by:

- Decentralizing student services
- Improving transparency
- Empowering students
- Enhancing trust
- Creating immutable records
- Encouraging accountability

We believe blockchain technology can improve collaboration, communication, and accountability within modern university ecosystems.

---

## Key Features

### 1. Campus Report Creation

- Create reports or academic tasks easily
- Add title, description, and category
- Automatic unique ID generation
- Instant storage on the Stellar blockchain

### 2. Retrieve All Reports

- Fetch all reports in a single smart contract call
- Structured output for frontend integration
- Real-time blockchain synchronization

### 3. Report Status Management

- Update report progress dynamically
- Track issue resolution transparently

Example statuses:
- OPEN
- PROCESS
- DONE

### 4. Secure Report Deletion

- Delete reports using unique IDs
- Automatic blockchain storage update

### 5. Blockchain Transparency & Security

- Immutable report records
- Secure decentralized storage
- Protected against unauthorized modification

### 6. Stellar & Soroban Integration

- Powered by Stellar blockchain
- Built using Soroban Smart Contract SDK
- Fast and low-cost transactions

---

## Smart Contract Functions

### `create_report()`
Create a new campus report or task.

### `get_reports()`
Retrieve all stored reports.

### `update_status()`
Update report progress or completion status.

### `delete_report()`
Delete a report using its unique ID.

---

## Tech Stack

- Rust Programming Language
- Soroban SDK
- Stellar Blockchain Network

---

## Getting Started

### 1. Clone Repository

```bash
git clone https://github.com/your-username/campus-board-dapp.git
cd campus-board-dapp
