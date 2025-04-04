// set up environment and agent, run training, then test learned policy
mod environment;
mod agent;
mod qlearning;

use environment::Environment;
use agent::Agent;
use qlearning::{train, test};

fn main() {
    // define grid dimensions and goal coords
    let env = Environment::new(5, 5, (4, 4));
    
    // alpha = learning rate, gamma = discount factor, epsilon = initial exploration rate
    let mut agent = Agent::new(5, 5, 0.1, 0.9, 1.0);
    
    let episodes = 1000;
    let max_steps = 50;
    
    train(&mut agent, &env, episodes, max_steps);
    
    println!("Testing learned policy:");
    test(&agent, &env, max_steps);
}
