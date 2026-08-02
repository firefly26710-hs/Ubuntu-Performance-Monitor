use ratatui::{
    widgets::{Block, Borders, Gauge, Paragraph},
    style::{Style, Color},
    layout::{Layout, Constraint, Direction},
    prelude::{Modifier},
    Frame,
};

use crate::a_data_source::data::DataSource;
pub fn gpu_present(f: &mut Frame, source: &mut DataSource) {
    let total = source.gauge_array[0];
    let avail = source.gauge_array[1];
    let used  = source.gauge_array[2];

   

    let total_ratio = 1.0;
    let avail_ratio = if total > 0.0 { (avail / total).clamp(0.0, 1.0) } else { 0.0 };
    let used_ratio  = if total > 0.0 { (used / total).clamp(0.0, 1.0) } else { 0.0 };

    let raw_name = std::str::from_utf8(&source.name_array).unwrap_or("");
    let gpu_name = raw_name.trim_matches(char::from(0)).trim();
    let cpu_information = format!("\n CPU NAME: {}\n ", gpu_name );

    let ascii_art = r#"




    @@@@@@@@@@@@@@@@   @@@@@@@       @@@@@@@ @@@@@@   @@@@@@@@@@@@@@@     @@@@@@         @@@@@@@@
    @@@@@@@@@@@@@@@@@@  @@@@@@@     @@@@@@@  @@@@@@   @@@@@@@@@@@@@@@@@@  @@@@@@       @@@@@@@@@@@
    @@@@@@@@@@@@@@@@@@@  @@@@@@     @@@@@@@  @@@@@@   @@@@@@@@@@@@@@@@@@  @@@@@@       @@@@@@@@@@@@
    @@@@@@       @@@@@@  @@@@@@@   @@@@@@@   @@@@@@   @@@@@@      @@@@@@  @@@@@@      @@@@@@@@@@@@@@
    @@@@@@       @@@@@@  @@@@@@@@  @@@@@@    @@@@@@   @@@@@@      @@@@@@@ @@@@@@      @@@@@@  @@@@@@
    @@@@@@       @@@@@@   @@@@@@@ @@@@@@     @@@@@@   @@@@@@      @@@@@@@ @@@@@@     @@@@@@    @@@@@@
    @@@@@@       @@@@@@    @@@@@@@@@@@@@     @@@@@@   @@@@@@     @@@@@@@  @@@@@@    @@@@@@@@@@@@@@@@@@
    @@@@@@       @@@@@@     @@@@@@@@@@@@     @@@@@@   @@@@@@@@@@@@@@@@@@  @@@@@@   @@@@@@@@@@@@@@@@@@@@
    @@@@@@       @@@@@@     @@@@@@@@@@@      @@@@@@   @@@@@@@@@@@@@@@@@@  @@@@@@   @@@@@@        @@@@@@@
    @@@@@@       @@@@@@      @@@@@@@@@       @@@@@@   @@@@@@@@@@@@@@@     @@@@@@  @@@@@@         @@@@@@@

    "#;

    let total_gauge = Gauge::default()
        .block(Block::default().title(" Total VRAM ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue).bg(Color::Black))
        .ratio(total_ratio)
        .label(format!("{:.1}G", total));

    let avail_gauge = Gauge::default()
        .block(Block::default().title(" Available VRAM ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .ratio(avail_ratio)
        .label(format!("{:.1}G", avail));

    let used_gauge = Gauge::default()
        .block(Block::default().title(" Used VRAM ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .ratio(used_ratio)
        .label(format!("{:.1}G", used));

    let gpu_paragraph = Paragraph::new(cpu_information)
        .block(Block::default().title(" GPU Information ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));



    let ascii_paragraph = Paragraph::new(ascii_art)
        .style(Style::default().fg(Color::Green));


    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ]).split(chunks[3]);

    // 4. 依序渲染上去，完美平行！
    f.render_widget(total_gauge, chunks[0]);
    f.render_widget(avail_gauge, chunks[1]);
    f.render_widget(used_gauge, chunks[2]);

    f.render_widget(gpu_paragraph, bottom_chunks[0]);
    f.render_widget(ascii_paragraph, bottom_chunks[1]);
}