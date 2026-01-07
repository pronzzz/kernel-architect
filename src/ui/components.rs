use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Widget},
};

use crate::pcg::automata::{Cluster, NodeType};

pub struct MapWidget<'a> {
    pub _active_tick: u64,
    pub player_x: u16,
    pub player_y: u16,
    pub cluster: &'a Cluster,
}

impl<'a> Widget for MapWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Neural Net Infrastructure ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Render Cluster
        let view_width = inner_area.width as usize;
        let view_height = inner_area.height as usize;
        
        for y in 0..self.cluster.height.min(view_height) {
            for x in 0..self.cluster.width.min(view_width) {
                 let node = &self.cluster.grid[y][x];
                 if node.node_type != NodeType::Void {
                     let (symbol, color) = match node.node_type {
                         NodeType::Server => ("▣", Color::Green),
                         NodeType::Database => ("▢", Color::Yellow),
                         NodeType::Firewall => ("║", Color::Red),
                         NodeType::Gateway => ("⎋", Color::Magenta),
                         NodeType::Void => (" ", Color::Reset),
                     };
                     
                     buf.set_string(inner_area.x + x as u16, inner_area.y + y as u16, symbol, Style::default().fg(color));
                 }
            }
        }
            
        // Draw Player (relative to inner area)
        let player_screen_x = inner_area.x + self.player_x;
        let player_screen_y = inner_area.y + self.player_y;
        
        // Bounds check against inner_area
        if player_screen_x < inner_area.x + inner_area.width && player_screen_y < inner_area.y + inner_area.height {
             buf.set_string(player_screen_x, player_screen_y, "@", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));
        }
    }
}

pub struct ConsoleWidget<'a> {
    pub logs: &'a [String],
}

impl<'a> Widget for ConsoleWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Kernel Log ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        
        let inner_area = block.inner(area);
        block.render(area, buf);

        for (i, log) in self.logs.iter().rev().take(inner_area.height as usize).enumerate() {
            buf.set_string(
                inner_area.x, 
                inner_area.y + inner_area.height - 1 - i as u16, 
                format!("> {}", log), 
                Style::default().fg(Color::Gray)
            );
        }
    }
}

pub struct StatusWidget {
    pub hp: i32,
    pub turn: i32,
}

impl Widget for StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = format!("HP: {} | Turn: {} | [Press 'q' to quit]", self.hp, self.turn);
        buf.set_string(area.x, area.y, content, Style::default().add_modifier(Modifier::BOLD));
    }
}

pub struct InfoWidget<'a> {
    pub info: &'a str,
}

impl<'a> Widget for InfoWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
         let block = Block::default()
            .title(" Node Info ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
         let inner = block.inner(area);
         block.render(area, buf);
         
         buf.set_string(inner.x, inner.y, self.info, Style::default().fg(Color::White));
         buf.set_string(inner.x, inner.y + 2, "[ENTER] to Hack", Style::default().fg(Color::Gray));
    }
}
