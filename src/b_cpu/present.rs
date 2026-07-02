use ratatui::{
    backend::Backend,
    widgets::{BarChart, Block, Borders, Gauge},

};
use ratatui::Frame;
use std::str::from_utf8;
use crate::a_data_source::data::{DataSource, PADDING_SIZE};
use crate::b_cpu::collection::{THREAD_NUMBER, THREAD_START};

pub fn draw_ui(f: &mut Frame, source: &mut DataSource) {
    let mut labels = Vec::new();
    for i in 0..THREAD_NUMBER {
        labels.push(format!("C{}", i));
    }

    let mut chart_data = Vec::new();

    for number in 0..THREAD_NUMBER {
        // 這裡直接從你的 history_array 裡面拿數值
        // 假設你的二維陣列結構是 [thread_index][history_index]
        // 或者是你用別的索引方式，這裡以最直接的 [number] 舉例：

        let usage_f64 = source.history_array[number][0]; // 拿最新的一筆歷史紀錄

        // BarChart 需要 u64，所以我們直接轉型
        let usage = (usage_f64 as u64).min(100);

        chart_data.push((labels[number].as_str(), usage));
    }

    // 2. 建立並渲染 BarChart
    let barchart = BarChart::default()
        .block(Block::default().title(" CPU Cores Load (%) ").borders(Borders::ALL))
        .data(&chart_data)
        .bar_width(3)
        .bar_gap(1);

    f.render_widget(barchart, f.size());
}