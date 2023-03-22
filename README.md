# Rust Network Monitor

## Group Name
NetWorks

## Group members - NetID
Vihaan Rao - vihaanr2 <br />
Rohit Choudhary - rohitkc2

## Project Intro
Our MVP is a Rust-based network monitoring tool. It is designed to help network administrators or developers monitor network traffic and diagnose network problems. The main goals and objectives of our project are:

- Monitor network traffic in real-time.
- Display information about network performance, including bandwidth usage, packet loss, and latency.
- Support multiple network interfaces for monitoring.
- Allow customization of monitoring parameters such as time intervals and network interface selection.
- Provide a simple, user-friendly command-line interface (CLI) for ease of use.

We chose to work on this project because we believe that a reliable and efficient network monitoring tool can be useful in various industries and fields. We also wanted to challenge ourselves to learn more about network protocols and Rust programming language.

## Technical overview
NetWorks is composed of the following major components

-  Network monitoring module: This module will use Rust's networking libraries to monitor network traffic on one or more network interfaces.
- Performance metrics module: This module will analyze network traffic data and calculate performance metrics such as bandwidth usage, packet loss, and latency.
- User interface module: This module will provide a user-friendly command-line interface (CLI) for users to interact with the tool.
- Configuration module: This module will allow users to customize monitoring parameters such as time intervals and network interface selection.

## Checkpoints
### Checkpoint 1:

- Implement network monitoring module to capture network traffic data.
- Implement performance metrics module to analyze captured network traffic data.
- Display basic performance metrics on the CLI.

### Checkpoint 2:

- Add support for multiple network interfaces.
- Allow customization of monitoring parameters such as time intervals and network interface selection.
- Display advanced performance metrics on the CLI.

### Checkpoint 3:

- Refactor code to improve readability and maintainability.
- Add support for logging and error handling.
- Create user documentation and help manual.

## Possible Challenges

- Learning and working with Rust's networking libraries.
- Designing an efficient and reliable performance metrics module.
- Creating a user-friendly CLI interface.

## References
- [Wireshark](https://www.wireshark.org/)
- [ntopng](https://www.ntop.org/products/traffic-analysis/ntop/)