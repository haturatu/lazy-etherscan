use crate::app::{statistics::Statistics, App};
use crate::widget::Spinner;
use ethers::core::utils::format_units;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: Rect) {
    let [right_statistics, left_statistics] = *Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(rect)
    else {
        return;
    };

    let [statistics0, statistics1, statistics2] = *Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(right_statistics)
    else {
        return;
    };

    let [statistics3, statistics4, statistics5] = *Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(left_statistics)
    else {
        return;
    };

    let statistic_items = [
        statistics0,
        statistics1,
        statistics2,
        statistics3,
        statistics4,
        statistics5,
    ];

    let statistic_titles = [
        "ETHER PRICE",
        "SUGGESTED BASE FEE",
        "LAST SAFE BLOCK",
        "NODE COUNT",
        "MED GAS PRICE",
        "LAST FINALIZED BLOCK",
    ];

    for (i, &statistic_item) in statistic_items.iter().enumerate() {
        let block = Block::default()
            .title(statistic_titles[i])
            .border_style(Style::default().fg(Color::White))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let [text_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Length(1)].as_ref())
            .split(statistic_item)
        else {
            return;
        };

        let text = if i == Statistics::ETHUSD_INDEX {
            if let Some(ethusd) = app.statistics.ethusd.as_ref() {
                format!("{:.4} USD/ETH", ethusd)
            } else {
                Spinner::default().to_string()
            }
        } else if i == Statistics::SUGGESTED_BASE_FEE_INDEX {
            if let Some(suggested_base_fee) = app.statistics.suggested_base_fee {
                format!("{} Gwei", format_units(suggested_base_fee, "gwei").unwrap())
            } else {
                Spinner::default().to_string()
            }
        } else if i == Statistics::NODE_COUNT_INDEX {
            if let Some(node_count) = app.statistics.node_count.as_ref() {
                format!("{node_count} nodes")
            } else {
                Spinner::default().to_string()
            }
        } else if i == Statistics::LAST_SAFE_BLOCK_INDEX {
            if let Some(block) = app.statistics.last_safe_block.as_ref() {
                format!("#{}", block.number.unwrap())
            } else {
                Spinner::default().to_string()
            }
        } else if i == Statistics::MED_GAS_PRICE_INDEX {
            if let Some(med_gas_price) = app.statistics.med_gas_price {
                format!("{} Gwei", format_units(med_gas_price, "gwei").unwrap())
            } else {
                Spinner::default().to_string()
            }
        } else if i == Statistics::LAST_FINALIZED_BLOCK_INDEX {
            if let Some(block) = app.statistics.last_finalized_block.as_ref() {
                format!("#{}", block.number.unwrap())
            } else {
                Spinner::default().to_string()
            }
        } else {
            Spinner::default().to_string()
        };

        let paragraph = Paragraph::new(vec![Line::from(Span::raw(text).fg(Color::White))])
            .block(block.to_owned())
            .alignment(Alignment::Right)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, text_rect);
        f.render_widget(block, statistic_item.to_owned());
    }
}
