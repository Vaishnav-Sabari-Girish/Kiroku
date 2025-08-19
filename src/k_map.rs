use crate::expr::Expr;
use crate::truth_table::variables;
use crate::eval::eval;
use std::collections::HashMap;
use comfy_table::{
    Table, Row, presets::UTF8_FULL,
    modifiers::UTF8_ROUND_CORNERS,
    ContentArrangement
};

// Generate Gray code sequence of n-bits 
fn gray_code(n: usize) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![vec![]];
    }

    let prev = gray_code(n - 1);
    let mut res = Vec::new();

    // Prefix with 0 
    for code in &prev {
        let mut new = vec![false];
        new.extend(code.clone());
        res.push(new);
    }
    // Prefix with 1
    for code in prev.iter().rev() {
        let mut new = vec![true];
        new.extend(code.clone());
        res.push(new);
    }

    res
}

// Generate K-Map for any expression for 2-6 variables
pub fn k_map(expr: &Expr) -> String {
    let vars = variables(expr);

    if vars.len() < 2 || vars.len() > 6 {
        return "K-Map supported only for 2 to 6 variables".to_string();
    }

    // Split into row and column variables
    let row_vars = vars[..vars.len() / 2].to_vec();
    let col_vars = vars[vars.len() / 2..].to_vec();

    let row_gray = gray_code(row_vars.len());
    let col_gray = gray_code(col_vars.len());

    // Prepare table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // ✅ FIXED: Proper header row
    let mut header_cells = vec![format!("{}\\{}", row_vars.join(""), col_vars.join(""))];
    for bits in &col_gray {
        let cell = bits.iter().map(|&b| if b { "1" } else { "0" }).collect::<String>();
        header_cells.push(cell);
    }
    table.set_header(Row::from(header_cells));

    // Rows
    for row in &row_gray {
        let mut row_cells = vec![row.iter().map(|&b| if b { "1" } else { "0" }).collect::<String>()];

        for col in &col_gray {
            let mut assignment = HashMap::new();

            for (v, &val) in row_vars.iter().zip(row.iter()) {
                assignment.insert(v.clone(), val);
            }
            for (v, &val) in col_vars.iter().zip(col.iter()) {
                assignment.insert(v.clone(), val);
            }

            let result = eval(expr, &assignment);
            row_cells.push(if result { "1".to_string() } else { "0".to_string() });
        }
        table.add_row(Row::from(row_cells));
    }

    table.to_string()
}
