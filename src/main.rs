use rand::seq::SliceRandom;

fn get_random_color_code() -> String {
    let colors = vec![
        "\x1b[31m", // Red
        "\x1b[32m", // Green
        "\x1b[33m", // Yellow
        "\x1b[34m", // Blue
        "\x1b[35m", // Magenta
        "\x1b[36m", // Cyan
    ];

    let selected_color = colors.choose(&mut rand::thread_rng()).unwrap();
    format!("{}", selected_color)
}

fn generate_waffle_chart(rows: usize, 
        columns: usize, 
        labels: Vec<&str>, 
        values: Vec<usize>, 
        asciis: Vec<char>) {

    let sum_values = values.iter().sum::<usize>();
    let total_cells = rows * columns;
    let mut label_shares = vec![];
    
    // iterate over the values and labels to calculate the share of each label
    for (_, &value) in labels.iter().zip(values.iter()) {
        let label_share = (value as f64 * total_cells as f64 / sum_values as f64)
            .round() as usize;
        
        // append each label share to lael_shares
        label_shares.push(label_share);
    }

    let chart = generate_shares_array(asciis, label_shares, columns, rows);
    show_chart(chart);
}

fn show_chart(chart: Vec<Vec<String>>) {
    for row in chart {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn generate_shares_array(asciis: Vec<char>, 
        label_shares: Vec<usize>, 
        columns: usize,
        rows: usize) -> Vec<Vec<String>>{
    let mut chart: Vec<Vec<String>> = vec![vec![" ".to_string(); columns]; rows];

    let mut current_row = 0;
    let mut current_col = 0;

    for (&ascii, &share) in asciis.iter().zip(label_shares.iter()) {
        let random_color = get_random_color_code();
        for _ in 0..share {

            chart[current_row][current_col] = format!("{}{}", random_color, ascii);

            current_col += 1;
            if current_col >= columns {
                current_col = 0;
                current_row += 1;
            }
        }
    }
    return chart;
}

// TODO: Select color for each label
// TODO: Create command line arguments to pass the values
// TODO: add horizontal and vertical paramenter to allow the user to choose the orientation of the chart

fn main() {
    let labels = vec!["A", "B", "C", "D"];
    let values = vec![30, 20, 20, 40];
    let characters = vec!['A', 'B', 'C', 'D'];
    generate_waffle_chart(4, 10, labels, values, characters);
}