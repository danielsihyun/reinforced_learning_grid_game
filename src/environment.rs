// grid environment, agent starts at 0,0
// 0 = up, 1 = right, 2 = down, 3 = left

pub struct Environment {
    pub width: usize,
    pub height: usize,
    pub goal: (usize, usize),
}

impl Environment {
    pub fn new(width: usize, height: usize, goal: (usize, usize)) -> Self {
        Environment { width, height, goal }
    }
    
    pub fn reset(&self) -> (usize, usize) {
        (0, 0)
    }
    
    // reward = -1 per step
    // if goal is reached, reward = 10
    pub fn step(&self, state: (usize, usize), action: u8) -> ((usize, usize), f64, bool) {
        let (x, y) = state;
        let mut new_x = x as isize;
        let mut new_y = y as isize;
        match action {
            0 => new_y -= 1,
            1 => new_x += 1,
            2 => new_y += 1,
            3 => new_x -= 1,
            _ => {}
        }
        if new_x < 0 { new_x = 0; }
        if new_y < 0 { new_y = 0; }
        if new_x >= self.width as isize { new_x = self.width as isize - 1; }
        if new_y >= self.height as isize { new_y = self.height as isize - 1; }
        
        let new_state = (new_x as usize, new_y as usize);
        let (reward, done) = if new_state == self.goal {
            (10.0, true)
        } else {
            (-1.0, false)
        };
        (new_state, reward, done)
    }
}
