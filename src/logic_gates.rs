use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    symbols,
    widgets::{Block, Borders, canvas::Canvas},
    Frame,
};
use crate::expr::Expr;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct GateInstance {
    pub gate_type: GateType,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<usize>,  // indices of input gates/variables
    pub output_connects_to: Vec<usize>, // indices of gates this output connects to
}

#[derive(Clone, Debug, PartialEq)]
pub enum GateType {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Xor,
    Xnor,
    Input(String), // Variable input
}

pub struct LogicGatesViewer {
    pub pan_x: f64,
    pub pan_y: f64,
    pub zoom: f64,
    gates: Vec<GateInstance>,
    variables: Vec<String>,
    expression: Option<Expr>,
}

impl LogicGatesViewer {
    pub fn new() -> Self {
        Self {
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
            gates: Vec::new(),
            variables: Vec::new(),
            expression: None,
        }
    }

    pub fn set_expression(&mut self, expr: Expr) {
        self.expression = Some(expr.clone());
        self.generate_circuit_from_expression(&expr);
    }

    fn generate_circuit_from_expression(&mut self, expr: &Expr) {
        self.gates.clear();
        self.variables.clear();
        
        // Collect all variables
        self.collect_variables(expr);
        
        // Create input gates for variables
        let mut gate_id_map = HashMap::new();
        for (i, var) in self.variables.iter().enumerate() {
            let input_gate = GateInstance {
                gate_type: GateType::Input(var.clone()),
                x: 20.0,
                y: 90.0 - (i as f64 * 25.0),
                width: 30.0,
                height: 15.0,
                inputs: Vec::new(),
                output_connects_to: Vec::new(),
            };
            self.gates.push(input_gate);
            gate_id_map.insert(format!("var_{}", var), self.gates.len() - 1);
        }
        
        // Build the circuit recursively
        let output_gate_id = self.build_circuit_recursive(expr, &mut gate_id_map, 80.0);
        
        // Position gates in layers
        self.layout_gates();
    }

    fn collect_variables(&mut self, expr: &Expr) {
        match expr {
            Expr::Var(name) => {
                if !self.variables.contains(name) {
                    self.variables.push(name.clone());
                }
            }
            Expr::And(left, right) | Expr::Or(left, right) | 
            Expr::Xor(left, right) | Expr::Nand(left, right) | 
            Expr::Nor(left, right) | Expr::Xnor(left, right) => {
                self.collect_variables(left);
                self.collect_variables(right);
            }
            Expr::Not(inner) => {
                self.collect_variables(inner);
            }
        }
    }

    fn build_circuit_recursive(&mut self, expr: &Expr, gate_id_map: &mut HashMap<String, usize>, x_pos: f64) -> usize {
        match expr {
            Expr::Var(name) => {
                // Return the index of the variable input gate
                *gate_id_map.get(&format!("var_{}", name)).unwrap()
            }
            Expr::And(left, right) => {
                self.create_binary_gate(GateType::And, left, right, gate_id_map, x_pos)
            }
            Expr::Or(left, right) => {
                self.create_binary_gate(GateType::Or, left, right, gate_id_map, x_pos)
            }
            Expr::Xor(left, right) => {
                self.create_binary_gate(GateType::Xor, left, right, gate_id_map, x_pos)
            }
            Expr::Nand(left, right) => {
                self.create_binary_gate(GateType::Nand, left, right, gate_id_map, x_pos)
            }
            Expr::Nor(left, right) => {
                self.create_binary_gate(GateType::Nor, left, right, gate_id_map, x_pos)
            }
            Expr::Xnor(left, right) => {
                self.create_binary_gate(GateType::Xnor, left, right, gate_id_map, x_pos)
            }
            Expr::Not(inner) => {
                let input_gate_id = self.build_circuit_recursive(inner, gate_id_map, x_pos - 50.0);
                
                let gate = GateInstance {
                    gate_type: GateType::Not,
                    x: x_pos,
                    y: 60.0, // Will be repositioned later
                    width: 30.0,
                    height: 15.0,
                    inputs: vec![input_gate_id],
                    output_connects_to: Vec::new(),
                };
                
                self.gates.push(gate);
                let gate_id = self.gates.len() - 1;
                
                // Update input gate to connect to this gate
                self.gates[input_gate_id].output_connects_to.push(gate_id);
                
                gate_id
            }
        }
    }

    fn create_binary_gate(&mut self, gate_type: GateType, left: &Expr, right: &Expr, 
                         gate_id_map: &mut HashMap<String, usize>, x_pos: f64) -> usize {
        let left_gate_id = self.build_circuit_recursive(left, gate_id_map, x_pos - 50.0);
        let right_gate_id = self.build_circuit_recursive(right, gate_id_map, x_pos - 50.0);
        
        let gate = GateInstance {
            gate_type,
            x: x_pos,
            y: 60.0, // Will be repositioned later
            width: 40.0,
            height: 20.0,
            inputs: vec![left_gate_id, right_gate_id],
            output_connects_to: Vec::new(),
        };
        
        self.gates.push(gate);
        let gate_id = self.gates.len() - 1;
        
        // Update input gates to connect to this gate
        self.gates[left_gate_id].output_connects_to.push(gate_id);
        self.gates[right_gate_id].output_connects_to.push(gate_id);
        
        gate_id
    }

    fn layout_gates(&mut self) {
        // Simple layered layout based on connectivity
        let mut layers: Vec<Vec<usize>> = Vec::new();
        let mut visited = vec![false; self.gates.len()];
        
        // Find input gates (variables) - these go in layer 0
        let mut current_layer = Vec::new();
        for (i, gate) in self.gates.iter().enumerate() {
            if matches!(gate.gate_type, GateType::Input(_)) {
                current_layer.push(i);
                visited[i] = true;
            }
        }
        if !current_layer.is_empty() {
            layers.push(current_layer);
        }
        
        // Build subsequent layers
        while layers.last().map_or(false, |layer| !layer.is_empty()) {
            let mut next_layer = Vec::new();
            
            for &gate_id in layers.last().unwrap() {
                for &connected_gate_id in &self.gates[gate_id].output_connects_to {
                    if !visited[connected_gate_id] {
                        // Check if all inputs of this gate are already visited
                        let all_inputs_ready = self.gates[connected_gate_id].inputs.iter()
                            .all(|&input_id| visited[input_id]);
                        
                        if all_inputs_ready {
                            next_layer.push(connected_gate_id);
                            visited[connected_gate_id] = true;
                        }
                    }
                }
            }
            
            if !next_layer.is_empty() {
                layers.push(next_layer);
            } else {
                break;
            }
        }
        
        // Position gates based on layers
        for (layer_idx, layer) in layers.iter().enumerate() {
            let x = 30.0 + (layer_idx as f64 * 70.0);
            let layer_height = layer.len() as f64 * 30.0;
            let start_y = (120.0 - layer_height) / 2.0;
            
            for (i, &gate_id) in layer.iter().enumerate() {
                self.gates[gate_id].x = x;
                self.gates[gate_id].y = start_y + (i as f64 * 30.0);
            }
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.pan_x -= dx / self.zoom;
        self.pan_y -= dy / self.zoom;
    }

    pub fn zoom_in(&mut self) {
        self.zoom *= 1.1;
        if self.zoom > 10.0 {
            self.zoom = 10.0;
        }
    }

    pub fn zoom_out(&mut self) {
        self.zoom /= 1.1;
        if self.zoom < 0.1 {
            self.zoom = 0.1;
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Logic Circuit"))
            .paint(|ctx| {
                ctx.layer();

                let transform = |x: f64, y: f64| ((x - self.pan_x) * self.zoom, (y - self.pan_y) * self.zoom);

                // Draw connections first (so they appear behind gates)
                for (gate_id, gate) in self.gates.iter().enumerate() {
                    let gate_output_x = gate.x + gate.width;
                    let gate_output_y = gate.y + gate.height / 2.0;
                    
                    for &target_gate_id in &gate.output_connects_to {
                        let target_gate = &self.gates[target_gate_id];
                        let target_input_x = target_gate.x;
                        
                        // Determine which input this connection goes to
                        let input_index = target_gate.inputs.iter().position(|&id| id == gate_id).unwrap_or(0);
                        let actual_target_y = if target_gate.inputs.len() > 1 {
                            target_gate.y + target_gate.height * (0.25 + input_index as f64 * 0.5)
                        } else {
                            target_gate.y + target_gate.height / 2.0
                        };
                        
                        let (x1, y1) = transform(gate_output_x, gate_output_y);
                        let (x2, y2) = transform(target_input_x, actual_target_y);
                        
                        // Draw connection line
                        ctx.draw(&ratatui::widgets::canvas::Line {
                            x1, y1, x2, y2,
                            color: Color::Green,
                        });
                    }
                }

                // Draw gates
                for gate in &self.gates {
                    let (x, y) = transform(gate.x, gate.y);
                    let width = gate.width * self.zoom;
                    let height = gate.height * self.zoom;
                    
                    match gate.gate_type {
                        GateType::And => Self::draw_and_gate(ctx, x, y, width, height),
                        GateType::Or => Self::draw_or_gate(ctx, x, y, width, height),
                        GateType::Not => Self::draw_not_gate(ctx, x, y, width, height),
                        GateType::Nand => Self::draw_nand_gate(ctx, x, y, width, height),
                        GateType::Nor => Self::draw_nor_gate(ctx, x, y, width, height),
                        GateType::Xor => Self::draw_xor_gate(ctx, x, y, width, height),
                        GateType::Xnor => Self::draw_xnor_gate(ctx, x, y, width, height),
                        GateType::Input(ref name) => {
                            Self::draw_input_gate(ctx, x, y, width, height);
                            // Draw variable label
                            ctx.print(x + width / 4.0, y + height / 2.0, name.clone());
                        }
                    }
                }

                // Draw gate labels
                for gate in &self.gates {
                    let (label_x, label_y) = transform(gate.x + gate.width / 2.0, gate.y - 5.0);
                    let label = match gate.gate_type {
                        GateType::And => "AND",
                        GateType::Or => "OR", 
                        GateType::Not => "NOT",
                        GateType::Nand => "NAND",
                        GateType::Nor => "NOR",
                        GateType::Xor => "XOR",
                        GateType::Xnor => "XNOR",
                        GateType::Input(_) => "",
                    };
                    if !label.is_empty() {
                        ctx.print(label_x - label.len() as f64 * 2.0, label_y, label.to_string());
                    }
                }
            })
            .marker(symbols::Marker::Braille)
            .x_bounds([0.0, 300.0])
            .y_bounds([0.0, 120.0]);

        f.render_widget(canvas, area);
    }

    // Input gate - simple rectangle
    fn draw_input_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        // Rectangle outline
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x, y1: y, x2: x + width, y2: y, color: Color::Cyan,
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x + width, y1: y, x2: x + width, y2: y + height, color: Color::Cyan,
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x + width, y1: y + height, x2: x, y2: y + height, color: Color::Cyan,
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x, y1: y + height, x2: x, y2: y, color: Color::Cyan,
        });
        
        // Output line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x + width,
            y1: y + height / 2.0,
            x2: x + width * 1.3,
            y2: y + height / 2.0,
            color: Color::Cyan,
        });
    }

    // AND Gate - shaped like a D
    fn draw_and_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        // Left vertical line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x,
            y1: y,
            x2: x,
            y2: y + height,
            color: Color::White,
        });
        // Top horizontal line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x,
            y1: y,
            x2: x + width * 0.6,
            y2: y,
            color: Color::White,
        });
        // Bottom horizontal line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x,
            y1: y + height,
            x2: x + width * 0.6,
            y2: y + height,
            color: Color::White,
        });

        // Right curved edge (semicircle)
        let center_y = y + height / 2.0;
        let radius = height / 2.0;
        let segments = 16;
        for i in 0..segments {
            let angle1 = -std::f64::consts::PI / 2.0 + (std::f64::consts::PI * i as f64) / segments as f64;
            let angle2 = -std::f64::consts::PI / 2.0 + (std::f64::consts::PI * (i + 1) as f64) / segments as f64;
            let x1 = x + width * 0.6 + radius * angle1.cos();
            let y1 = center_y + radius * angle1.sin();
            let x2 = x + width * 0.6 + radius * angle2.cos();
            let y2 = center_y + radius * angle2.sin();
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1, y1, x2, y2,
                color: Color::White,
            });
        }

        // Input lines
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x - width * 0.2,
            y1: y + height * 0.25,
            x2: x,
            y2: y + height * 0.25,
            color: Color::White,
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x - width * 0.2,
            y1: y + height * 0.75,
            x2: x,
            y2: y + height * 0.75,
            color: Color::White,
        });
        // Output line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x + width,
            y1: center_y,
            x2: x + width * 1.2,
            y2: center_y,
            color: Color::White,
        });
    }

    // OR Gate - curved shape
    fn draw_or_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        let center_y = y + height / 2.0;
        
        // Left curved edge
        let segments = 10;
        for i in 0..segments {
            let t1 = i as f64 / segments as f64;
            let t2 = (i + 1) as f64 / segments as f64;
            let x1 = x + width * 0.2 * t1;
            let y1 = y + height * (0.5 - 0.4 * (1.0 - t1).powf(0.5));
            let x2 = x + width * 0.2 * t2;
            let y2 = y + height * (0.5 - 0.4 * (1.0 - t2).powf(0.5));
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
            let y1 = y + height * (0.5 + 0.4 * (1.0 - t1).powf(0.5));
            let y2 = y + height * (0.5 + 0.4 * (1.0 - t2).powf(0.5));
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }
        
        // Right curved edge
        for i in 0..segments {
            let angle1 = -std::f64::consts::PI / 3.0 + (2.0 * std::f64::consts::PI / 3.0 * i as f64) / segments as f64;
            let angle2 = -std::f64::consts::PI / 3.0 + (2.0 * std::f64::consts::PI / 3.0 * (i + 1) as f64) / segments as f64;
            let radius = height * 0.6;
            let x1 = x + width * 0.4 + radius * angle1.cos();
            let y1 = center_y + radius * angle1.sin();
            let x2 = x + width * 0.4 + radius * angle2.cos();
            let y2 = center_y + radius * angle2.sin();
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }
        
        // Input lines
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x - width * 0.2,
            y1: y + height * 0.25,
            x2: x + width * 0.1,
            y2: y + height * 0.3,
            color: Color::White,
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x - width * 0.2,
            y1: y + height * 0.75,
            x2: x + width * 0.1,
            y2: y + height * 0.7,
            color: Color::White,
        });
        // Output line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x + width,
            y1: center_y,
            x2: x + width * 1.2,
            y2: center_y,
            color: Color::White,
        });
    }

    // NOT Gate - triangle with circle
    fn draw_not_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        // Triangle body
        ctx.draw(&ratatui::widgets::canvas::Line { 
            x1: x, 
            y1: y, 
            x2: x, 
            y2: y + height, 
            color: Color::White 
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x,
            y1: y,
            x2: x + width * 0.8,
            y2: y + height / 2.0,
            color: Color::White,
        });
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x,
            y1: y + height,
            x2: x + width * 0.8,
            y2: y + height / 2.0,
            color: Color::White,
        });

        // Inversion circle
        let circle_center_x = x + width * 0.85;
        let circle_center_y = y + height / 2.0;
        let circle_radius = width * 0.05;
        let segments = 12;
        for i in 0..segments {
            let angle1 = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let angle2 = 2.0 * std::f64::consts::PI * (i + 1) as f64 / segments as f64;
            let x1 = circle_center_x + circle_radius * angle1.cos();
            let y1 = circle_center_y + circle_radius * angle1.sin();
            let x2 = circle_center_x + circle_radius * angle2.cos();
            let y2 = circle_center_y + circle_radius * angle2.sin();
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }

        // Input line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x - width * 0.2,
            y1: y + height / 2.0,
            x2: x,
            y2: y + height / 2.0,
            color: Color::White,
        });
        // Output line
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: x + width,
            y1: y + height / 2.0,
            x2: x + width * 1.2,
            y2: y + height / 2.0,
            color: Color::White,
        });
    }

    // NAND Gate - AND gate with inversion circle
    fn draw_nand_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        Self::draw_and_gate(ctx, x, y, width * 0.9, height);
        
        // Inversion circle
        let circle_center_x = x + width * 0.95;
        let circle_center_y = y + height / 2.0;
        let circle_radius = width * 0.05;
        let segments = 12;
        for i in 0..segments {
            let angle1 = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let angle2 = 2.0 * std::f64::consts::PI * (i + 1) as f64 / segments as f64;
            let x1 = circle_center_x + circle_radius * angle1.cos();
            let y1 = circle_center_y + circle_radius * angle1.sin();
            let x2 = circle_center_x + circle_radius * angle2.cos();
            let y2 = circle_center_y + circle_radius * angle2.sin();
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }
    }

    // XOR Gate - OR gate with additional curved line
    fn draw_xor_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        Self::draw_or_gate(ctx, x, y, width, height);
        
        // Additional curved line at the back
        let segments = 8;
        for i in 0..segments {
            let t1 = i as f64 / segments as f64;
            let t2 = (i + 1) as f64 / segments as f64;
            let offset = width * 0.1;
            let x1 = x - offset + width * 0.15 * t1;
            let y1 = y + height * (0.5 - 0.35 * (1.0 - t1).powf(0.5));
            let x2 = x - offset + width * 0.15 * t2;
            let y2 = y + height * (0.5 - 0.35 * (1.0 - t2).powf(0.5));
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
            let y1 = y + height * (0.5 + 0.35 * (1.0 - t1).powf(0.5));
            let y2 = y + height * (0.5 + 0.35 * (1.0 - t2).powf(0.5));
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }
    }

    // NOR Gate - OR gate with inversion circle
    fn draw_nor_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        Self::draw_or_gate(ctx, x, y, width * 0.9, height);
        
        // Inversion circle
        let circle_center_x = x + width * 0.95;
        let circle_center_y = y + height / 2.0;
        let circle_radius = width * 0.05;
        let segments = 12;
        for i in 0..segments {
            let angle1 = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let angle2 = 2.0 * std::f64::consts::PI * (i + 1) as f64 / segments as f64;
            let x1 = circle_center_x + circle_radius * angle1.cos();
            let y1 = circle_center_y + circle_radius * angle1.sin();
            let x2 = circle_center_x + circle_radius * angle2.cos();
            let y2 = circle_center_y + circle_radius * angle2.sin();
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }
    }

    // XNOR Gate - XOR gate with inversion circle
    fn draw_xnor_gate(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, width: f64, height: f64) {
        Self::draw_xor_gate(ctx, x, y, width * 0.9, height);
        
        // Inversion circle
        let circle_center_x = x + width * 0.95;
        let circle_center_y = y + height / 2.0;
        let circle_radius = width * 0.05;
        let segments = 12;
        for i in 0..segments {
            let angle1 = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let angle2 = 2.0 * std::f64::consts::PI * (i + 1) as f64 / segments as f64;
            let x1 = circle_center_x + circle_radius * angle1.cos();
            let y1 = circle_center_y + circle_radius * angle1.sin();
            let x2 = circle_center_x + circle_radius * angle2.cos();
            let y2 = circle_center_y + circle_radius * angle2.sin();
            ctx.draw(&ratatui::widgets::canvas::Line { x1, y1, x2, y2, color: Color::White });
        }
    }
}
