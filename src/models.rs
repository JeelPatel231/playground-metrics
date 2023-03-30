#[derive(Debug)]
pub enum Metric {
    Weight,
    Length,
    Area,
    Volume,
    Charge,
    Energy,
    Currency,
}

#[derive(Debug, PartialEq)]
pub enum Position {
    BeforeAmount,
    AfterAmount,
    Both,
}

#[derive(Debug)]
pub struct Unit {
    pub metric: Metric,
    pub position: Position,
    pub factor: f64,
}

// 191g
const DAVINCI_KG: f64 = 0.191;
// 156.7mm
const DAVINCI_M: f64 = 0.1567;
// 156.7mm * 74.3mm
const DAVINCI_SQM: f64 = 0.011_642_81;
// 156.7mm * 74.3mm * 8.8mm
const DAVINCI_CM: f64 = 0.000_102_456_7;
// 400mAh
const DAVINCI_AH: f64 = 4.0;
// 15.4Wh
const DAVINCI_J: f64 = 55440.0;
// 1999CNY on release
const DAVINCI_CNY: f64 = 1999.0;

impl Unit {
    pub fn in_davincis(&self, amount: f64) -> f64 {
        let amount = amount / self.factor;

        match self.metric {
            Metric::Weight => amount / DAVINCI_KG,
            Metric::Length => amount / DAVINCI_M,
            Metric::Area => amount / DAVINCI_SQM,
            Metric::Volume => amount / DAVINCI_CM,
            Metric::Charge => amount / DAVINCI_AH,
            Metric::Energy => amount / DAVINCI_J,
            Metric::Currency => amount / DAVINCI_CNY,
        }
    }
}
