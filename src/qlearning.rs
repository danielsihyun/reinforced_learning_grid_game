// implement training and testing loops for q learning
use crate::agent::Agent;
use crate::environment::Environment;

pub fn train(agent: &mut Agent, env: &Environment, episodes: usize, max_steps: usize) {
    for episode in 0..episodes {
        let mut state = env.reset();
        let mut total_reward = 0.0;
        for _ in 0..max_steps {
            let action = agent.choose_action(state);
            let (next_state, reward, done) = env.step(state, action);
            agent.update_q(state, action, reward, next_state);
            state = next_state;
            total_reward += reward;
            if done {
                break;
            }
        }
        if agent.epsilon > 0.1 {
            agent.epsilon *= 0.99;
        }
        println!("Episode {}: Total Reward = {}", episode, total_reward);
    }
}

pub fn test(agent: &Agent, env: &Environment, max_steps: usize) {
    let mut state = env.reset();
    println!("Testing learned policy starting from state: {:?}", state);
    for step in 0..max_steps {
        let action = agent.choose_action(state);
        let (next_state, reward, done) = env.step(state, action);
        println!("Step {}: State: {:?}, Action: {}, Reward: {}, Next State: {:?}", step, state, action, reward, next_state);
        state = next_state;
        if done {
            println!("Reached goal state!");
            break;
        }
    }
}
