use ratatui::{
    widgets::{Block, Borders, Gauge},
    style::{Style, Color},
    layout::{Layout, Constraint, Direction},
    Frame,
};
use crate::a_data_source::data::DataSource;

pub fn disk_ui(f: &mut Frame, source: &mut DataSource) {
    let total_kb = source.gauge_array[0];
    let avail_kb = source.gauge_array[1];
    let used_kb  = source.gauge_array[2];

    let total_gb = total_kb / 1024.0 / 1024.0 / 1024.0;
    let avail_gb = avail_kb / 1024.0 / 1024.0 / 1024.0;
    let used_gb  = used_kb / 1024.0 / 1024.0 / 1024.0;

    let total_ratio = 1.0;
    let avail_ratio = if total_gb > 0.0 { (avail_gb / total_gb).clamp(0.0, 1.0) } else { 0.0 };
    let used_ratio  = if total_gb > 0.0 { (used_gb / total_gb).clamp(0.0, 1.0) } else { 0.0 };

    // 2. 建立三根不同的 Gauge
    let total_gauge = Gauge::default()
        .block(Block::default().title(" Total Memory ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue).bg(Color::Black))
        .ratio(total_ratio)
        .label(format!("{:.1}G", total_gb));

    let avail_gauge = Gauge::default()
        .block(Block::default().title(" Available Memory ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .ratio(avail_ratio)
        .label(format!("{:.1}G", avail_gb));

    let used_gauge = Gauge::default()
        .block(Block::default().title(" Used Memory ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .ratio(used_ratio)
        .label(format!("{:.1}G", used_gb));

    // 3. 🎯 核心排版：把全螢幕高度切成三等分 (每根 Gauge 佔 3 行高度)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 第一根 Total
            Constraint::Length(3), // 第二根 Avail
            Constraint::Length(3), // 第三根 Used
            //Constraint::Min(0),    // 剩餘留白
        ])
        .split(f.size());

    // 4. 依序渲染上去，完美平行！
    f.render_widget(total_gauge, chunks[0]);
    f.render_widget(avail_gauge, chunks[1]);
    f.render_widget(used_gauge, chunks[2]);
}