#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Exploration,
    KernelDebug,
    LogicCombat,
    Inventory,
    // Add a Menu or Title state if needed
    Title,
}

#[allow(dead_code)]
pub struct App {
    pub should_quit: bool,
    pub state: GameState,
    pub tick_rate: u64,
    pub animation_tick: u64,
    pub player_x: u16,
    pub player_y: u16,
    pub cluster: crate::pcg::automata::Cluster,
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let mut cluster = crate::pcg::automata::Cluster::new(60, 30);
        cluster.generate();
        
        Self {
            should_quit: false,
            state: GameState::Exploration,
            tick_rate: 250,
            animation_tick: 0,
            player_x: 2,
            player_y: 2,
            cluster,
            logs: vec!["System initialized.".to_string()],
        }
    }

    pub fn log(&mut self, msg: String) {
        self.logs.push(msg);
        if self.logs.len() > 10 {
            self.logs.remove(0);
        }
    }

    pub fn on_tick(&mut self) {
        self.animation_tick = self.animation_tick.wrapping_add(1);
    }

    pub fn move_player(&mut self, dx: i16, dy: i16) {
        let new_x = (self.player_x as i16 + dx).max(0) as u16;
        let new_y = (self.player_y as i16 + dy).max(0) as u16;
        
        // Bounds check (relative to cluster size now)
        if (new_x as usize) < self.cluster.width && (new_y as usize) < self.cluster.height {
            self.player_x = new_x;
            self.player_y = new_y;
        }
    }

    pub fn get_current_node_info(&self) -> String {
        let x = self.player_x as usize;
        let y = self.player_y as usize;
        if x < self.cluster.width && y < self.cluster.height {
             let node = &self.cluster.grid[y][x];
             match node.node_type {
                 crate::pcg::automata::NodeType::Void => "Sector Empty".to_string(),
                 t => format!("{:?} [Int: {}%]", t, node.integrity),
             }
        } else {
            "Out of Bounds".to_string()
        }
    }
    
    pub fn hack_current_node(&mut self) -> Option<String> {
        let x = self.player_x as usize;
        let y = self.player_y as usize;
        if x < self.cluster.width && y < self.cluster.height {
             let node = &mut self.cluster.grid[y][x];
             if node.node_type != crate::pcg::automata::NodeType::Void {
                 if !node.is_compromised {
                     node.is_compromised = true;
                     node.integrity = 0;
                     return Some(format!("HACKED: {:?} at {},{}", node.node_type, x, y));
                 } else {
                     return Some("Already compromised.".to_string());
                 }
             }
        }
        None
    }
}
