use ratatui::{
    layout::{Layout, Constraint},
    widgets::{BarChart, Block, Borders, Paragraph},
    style::{Style, Color, Modifier},
    Frame,
};
use crate::a_data_source::data::{DataSource};
use crate::b_cpu::collection::THREAD_NUMBER;

const CPU_LABELS: [&str; 12] = ["C0", "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11"];
pub fn draw_ui(f: &mut Frame, source: &mut DataSource) {
    let mut data: [(&str, u64); 12] = [("", 0); 12];


    let chart_array = &mut source.chart_array;
    for(number,value) in chart_array.iter().enumerate(){
        let usage_as_f64 = value;
        let usage = (*value as u64).min(100);
        data[number] = (CPU_LABELS[number], usage);
    }

    let barchart = BarChart::default()
        .block(Block::default().title(" CPU Cores Load (%) ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Blue))
        .data(&data)
        .bar_width(3)
        .bar_gap(5);

    let raw_name = std::str::from_utf8(&source.name_array).unwrap_or("");
    let cpu_name = raw_name.trim_matches(char::from(0)).trim();

    let cpu_information = format!("\n CPU NAME: {}\n Cores: {} ({} Threads)", cpu_name, THREAD_NUMBER/2, THREAD_NUMBER, );
    let cpu_paragraph = Paragraph::new(cpu_information)
        .block(Block::default().title(" CPU Information ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

    let chunks = Layout::horizontal([
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ]).split(f.size());


    f.render_widget(barchart, chunks[0]);
    f.render_widget(cpu_paragraph, chunks[1]);
}