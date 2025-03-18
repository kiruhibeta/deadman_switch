# Deadman Switch Email Notifier

This Rust project implements a "deadman switch" system that sends email notifications when the user fails to check in within a specified time frame. The system checks a specified "check-in" file to determine if the user has sent a recent check-in. If the check-in is overdue, it triggers an email alert to the user's family or contacts, notifying them of the situation.

## Features
- Sends reminder emails if the user hasn't checked in within a specified number of days.
- Activates a deadman switch if the user has failed to check in beyond a certain threshold, notifying family members via email.
- Configurable via a `config.json` file.
- Simple email sending using SMTP protocol via the `lettre` crate.

## Prerequisites
Before running the program, ensure you have the following:
- A working SMTP server (such as Gmail, SendGrid, or your own SMTP server).
- A `config.json` file with the necessary configuration parameters.
- Rust installed on your system.

## Installation

### 1. Install Rust
If you don't have Rust installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust.

### 2. Clone the Repository
Clone this repository to your local machine:
```bash
git clone https://github.com/yourusername/deadman_switch.git
cd deadman_switch

3. Install Dependencies

Run the following command to build the project and install its dependencies:

