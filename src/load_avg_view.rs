use std::{collections::VecDeque, vec};

use iced::{Canvas, Container, Element, Length, Rectangle, canvas::{self, Cache, Cursor, Geometry}};
use plotters::{prelude::{ChartBuilder, IntoDrawingArea, LabelAreaPosition, LineSeries}, style::{BLUE, GREEN, RED}};

use crate::{custom_plot_backend::{CustomPlotFrame, Plottable}};
use crate::models;

const MAX_PLOT_SIZE: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LoadAvgValue {
    one_m: f64,
    five_m: f64,
    fifteen_m: f64
}

struct Graph {
    cache: Cache,
    load_avg: VecDeque<LoadAvgValue>
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
            load_avg: VecDeque::from(vec![LoadAvgValue::default(); MAX_PLOT_SIZE])
        }
    }
}

#[derive(Default)]
pub struct LoadAvgView {
    graph: Graph
}

#[derive(Debug, Clone, Copy)]
pub enum LoadAvgMessage {
}

impl LoadAvgView {
    fn round(avg: f64, digit: f64) -> f64 {
        (avg * digit).round() / digit
    }

    pub fn push_load_avg(&mut self, load_avg: &sysinfo::LoadAvg) {
        let one_m = Self::round(load_avg.one, 100.0);
        let five_m = Self::round(load_avg.five, 100.0);
        let fifteen_m = Self::round(load_avg.fifteen, 100.0);

        self.graph.load_avg.pop_front();
        self.graph.load_avg.push_back(
            LoadAvgValue {
                one_m,
                five_m,
                fifteen_m
            }
        );
    }
}

impl models::GraphView for LoadAvgView {
    type Message = LoadAvgMessage;
    
    fn clear_canvas_cache(&mut self) {
        self.graph.cache.clear();
    }

    fn update(&mut self, _msg: Self::Message) {
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Canvas::new(&mut self.graph)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<LoadAvgMessage> canvas::Program<LoadAvgMessage> for Graph {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let plotter_geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_plot(CustomPlotFrame::new(frame))
        });

        vec![
            plotter_geometry
        ]
    }
}

impl Plottable for Graph {
    fn draw_plot(&self, f: CustomPlotFrame) {
        let root_draw_area = f.into_drawing_area();

        let y_spec_end = match self.load_avg.get(self.load_avg.len() - 1)  {
            Some(val) => val.one_m + 2.5,
            None => 1.0
        };

        let mut ctx = ChartBuilder::on(&root_draw_area)
            .margin_top(20)
            .margin_bottom(20)
            .margin_right(20)
            .margin_left(10)
            .caption("Load Average R - 1m / G - 5m / B - 15m", ("Arial",24))
            .set_label_area_size(LabelAreaPosition::Left, 20)
            // .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .build_cartesian_2d(0..(self.load_avg.len() - 1), 0.0..y_spec_end)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(
            LineSeries::new(
                (0..).zip(self.load_avg.iter()).filter_map(|(i, val)| {
                    if val.one_m != 0.0 { Some((i, val.one_m )) } else { None }
                }),
                &RED
            )
        )
        .unwrap();

        ctx.draw_series(
            LineSeries::new(
                (0..).zip(self.load_avg.iter()).filter_map(|(i, val)| {
                    if val.five_m != 0.0 { Some((i, val.five_m)) } else { None }
                }),
                &GREEN
            )
        )
        .unwrap();

        ctx.draw_series(
            LineSeries::new(
                (0..).zip(self.load_avg.iter()).filter_map(|(i, val)| {
                    if val.fifteen_m != 0.0 { Some((i, val.fifteen_m)) } else { None }
                }),
                &BLUE
            )
        )
        .unwrap(); 
    }
}