use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    symbols,
    widgets::{Block, Borders, canvas::Canvas},
    Frame,
};

pub struct LogicGatesViewer {
    pan_x: f64,
    pan_y: f64,
    zoom: f64,
}

impl LogicGatesViewer {
    pub fn new() -> Self {
        Self {
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
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
            .block(Block::default().borders(Borders::ALL).title("Logic Gates"))
            .paint(|ctx| {
                ctx.layer();

                let transform = |x: f64, y: f64| ((x - self.pan_x) * self.zoom, (y - self.pan_y) * self.zoom);

                // Draw gates in a grid layout
                let (x, y) = transform(10.0, 90.0);
                Self::draw_and_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);
                let (x, y) = transform(80.0, 90.0);
                Self::draw_or_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);
                let (x, y) = transform(150.0, 90.0);
                Self::draw_not_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);
                let (x, y) = transform(10.0, 60.0);
                Self::draw_nand_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);
                let (x, y) = transform(80.0, 60.0);
                Self::draw_nor_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);
                let (x, y) = transform(10.0, 30.0);
                Self::draw_xor_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);
                let (x, y) = transform(80.0, 30.0);
                Self::draw_xnor_gate(ctx, x, y, 40.0 * self.zoom, 20.0 * self.zoom);

                let mut print_label = |x: f64, y: f64, text: String| {
                    let (px, py) = transform(x, y);
                    ctx.print(px, py, text);
                };

                // Add labels
                print_label(15.0, 85.0, "AND".to_string());
                print_label(85.0, 85.0, "OR".to_string());
                print_label(155.0, 85.0, "NOT".to_string());
                print_label(15.0, 55.0, "NAND".to_string());
                print_label(85.0, 55.0, "NOR".to_string());
                print_label(15.0, 25.0, "XOR".to_string());
                print_label(85.0, 25.0, "XNOR".to_string());
            })
            .marker(symbols::Marker::Braille)
            .x_bounds([0.0, 200.0])
            .y_bounds([0.0, 120.0]);

        f.render_widget(canvas, area);
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
