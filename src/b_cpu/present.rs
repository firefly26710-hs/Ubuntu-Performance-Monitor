use ratatui::{
    layout::{Layout, Constraint},
    widgets::{BarChart, Block, Borders, Paragraph},
    style::{Style, Color, Modifier},
    Frame,
};
use ratatui::layout::Direction;
use crate::a_data_source::data::{DataSource};
use crate::a_data_source::hpl::thread_number;
use crate::b_cpu::collection::THREAD_NUMBER;

const CPU_LABELS: [&str; 12] = ["T0", "T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "T9", "T10", "T11"];
pub fn cpu_present(f: &mut Frame, source: &mut DataSource, thread_number : usize) {
    let mut data: [(&str, u64); 12] = [("", 0); 12];
    let raw_name = std::str::from_utf8(&source.name_array).unwrap_or("");
    let cpu_name = raw_name.trim_matches(char::from(0)).trim();
    let cpu_information = format!("\n CPU NAME: {}\n Cores: {} ({} Threads)", cpu_name, THREAD_NUMBER/2, THREAD_NUMBER, );

    let chart_array = &mut source.chart_array;
    for(number,value) in chart_array.iter().take(thread_number).enumerate(){
        let usage = (*value as u64).min(100);
        data[number] = (CPU_LABELS[number], usage);
    }

    let ascii_logo = r#"



       @@@@        @@@@     @@@@  @@@@@@@@@     @@@@@@@@@@@@@
      @@@@@@       @@@@@@  @@@@@  @@@@@@@@@@@     @@@@@@@@@@@
     @@@@@@@@      @@@@@@@@@@@@@  @@@    @@@@@    @     @@@@@
    @@@@  @@@@     @@@@@@@@@ @@@  @@@     @@@@  @@@     @@@@@
   @@@@@@@@@@@@    @@@@  @@  @@@  @@@    @@@@@ @@@@     @@@@@
  @@@@@@@@@@@@@@   @@@@      @@@  @@@@@@@@@@@  @@@@@@@@@ @@@@
  @@@@@     @@@@   @@@@      @@@  @@@@@@@@@    @@@@@@@@    @@
    "#;



    let barchart = BarChart::default()
        .block(Block::default().title(" CPU Cores Load (%) ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Blue))
        .data(&data)
        .bar_width(3)
        .bar_gap(7);


    let cpu_paragraph = Paragraph::new(cpu_information)
        .block(Block::default().title(" CPU Information ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

    let ascii_paragraph = Paragraph::new(ascii_logo)
        .style(Style::default().fg(Color::Indexed(208)));

    
    let chunks = Layout::horizontal([
        Constraint::Percentage(65),
        Constraint::Percentage(35),
    ]).split(f.area());

    let bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), // 左邊：GPU 資訊
            Constraint::Percentage(70), // 右邊：ASCII 藝術
        ]).split(chunks[1]);


    f.render_widget(barchart, chunks[0]);

    f.render_widget(cpu_paragraph, bottom_chunks[0]);
    f.render_widget(ascii_paragraph, bottom_chunks[1]);
}