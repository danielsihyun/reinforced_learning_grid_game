// defines agent (uses q table for learning, epsilon greedy policy and updates q values)
use rand::Rng;

pub struct Agent {
    pub q_table: Vec<Vec<Vec<f64>>>,
    pub alpha: f64,
    pub gamma: f64,
    pub epsilon: f64,
}

impl Agent {
    pub fn new(width: usize, height: usize, alpha: f64, gamma: f64, epsilon: f64) -> Self {
        let q_table = vec![vec![vec![0.0; 4]; height]; width];
        Agent { q_table, alpha, gamma, epsilon }
    }
    
    pub fn choose_action(&self, state: (usize, usize)) -> u8 {
        let mut rng = rand::thread_rng();
        if rng.r#gen::<f64>() < self.epsilon {
            rng.gen_range(0..4) as u8
        } else {
            let (x, y) = state;
            let actions = &self.q_table[x][y];
            let mut max_index = 0;
            let mut max_value = actions[0];
            for (i, &value) in actions.iter().enumerate() {
                if value > max_value {
                    max_value = value;
                    max_index = i;
                }
            }
            max_index as u8
        }
    }
    
    pub fn update_q(&mut self, state: (usize, usize), action: u8, reward: f64, next_state: (usize, usize)) {
        let (x, y) = state;
        let (nx, ny) = next_state;
        let max_next = self.q_table[nx][ny].iter().fold(f64::MIN, |a, &b| a.max(b));
        let current_q = self.q_table[x][y][action as usize];
        let new_q = current_q + self.alpha * (reward + self.gamma * max_next - current_q);
        self.q_table[x][y][action as usize] = new_q;
    }
}
