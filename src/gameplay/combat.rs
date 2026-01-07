#![allow(dead_code)]
use crate::pcg::automata::Node;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CombatPhase {
    Initiative,
    PlayerTurn,
    EnemyTurn,
    Resolution,
}

#[allow(dead_code)]
pub struct CombatState {
    pub target_node: Node,
    pub player_hp: i32,
    pub enemy_hp: i32,
    pub turn: i32,
    pub phase: CombatPhase,
    pub log: Vec<String>,
}

impl CombatState {
    pub fn new(target_node: Node) -> Self {
        Self {
            target_node,
            player_hp: 100,
            enemy_hp: 50, // Simplified
            turn: 1,
            phase: CombatPhase::Initiative,
            log: vec!["Combat initialization sequence start...".to_string()],
        }
    }

    pub fn execute_player_action(&mut self, action: &str) {
        self.log.push(format!("Player executes: {}", action));
        // Simplified damage logic
        let damage = 10;
        self.enemy_hp -= damage;
        self.log.push(format!("Did {} logic damage to node.", damage));
        
        if self.enemy_hp <= 0 {
            self.phase = CombatPhase::Resolution;
            self.log.push("Target node neutralized. Architecture secured.".to_string());
        } else {
            self.phase = CombatPhase::EnemyTurn;
            self.enemy_turn();
        }
    }

    fn enemy_turn(&mut self) {
        self.log.push("Enemy countermeasures active...".to_string());
        let damage = 5;
        self.player_hp -= damage;
        self.log.push(format!("Took {} integrity damage.", damage));
        self.turn += 1;
        self.phase = CombatPhase::PlayerTurn;
    }
}
