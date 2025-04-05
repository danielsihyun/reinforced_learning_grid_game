Overview

This project demonstrates a reinforcement learning agent applied to a simple grid world environment. The environment consists of a 5x5 grid with a defined goal state. The agent uses Q-learning with an epsilon-greedy policy to learn how to navigate from the start state (0,0) to the goal (4,4).

Features

• A grid world environment that provides state transitions and rewards. • A Q-learning agent that maintains a Q-table for state-action pairs. • A training loop that updates the agent’s policy over multiple episodes with epsilon decay. • A test routine that demonstrates the learned policy.

Project Structure

• Cargo.toml - Contains project configuration and dependencies. • src/environment.rs - Defines the grid world, including state transitions, rewards, and terminal conditions. • src/agent.rs - Implements the Q-learning agent with an epsilon-greedy policy. • src/qlearning.rs - Contains the training and testing loops. • src/main.rs - Entry point that sets up the environment and agent, and orchestrates training and testing.

Installation and Running

Install Rust and Cargo.
Clone the repository and navigate to the project folder.
Build and run the project using the command: cargo run
Training Parameters

You can adjust parameters such as the number of episodes, maximum steps per episode, learning rate (alpha), discount factor (gamma), and the epsilon decay rate in the source code (typically in main.rs and qlearning.rs).
