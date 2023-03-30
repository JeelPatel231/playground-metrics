use crate::{Metric, Position, Unit};

pub static KILOGRAM: Unit = Unit {
    metric: Metric::Weight,
    position: Position::AfterAmount,
    factor: 1.0,
};
pub static GRAM: Unit = Unit {
    metric: Metric::Weight,
    position: Position::AfterAmount,
    factor: 1000.0,
};
pub static MILLIGRAM: Unit = Unit {
    metric: Metric::Weight,
    position: Position::AfterAmount,
    factor: 1_000_000.0,
};
pub static MICROGRAM: Unit = Unit {
    metric: Metric::Weight,
    position: Position::AfterAmount,
    factor: 1_000_000_000.0,
};
pub static TON: Unit = Unit {
    metric: Metric::Weight,
    position: Position::AfterAmount,
    factor: 0.001,
};

pub static METRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 1.0,
};
pub static DECIMETRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 10.0,
};
pub static CENTIMETRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 100.0,
};
pub static MILLIMETRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 1000.0,
};
pub static MICROMETRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 1_000_000.0,
};
pub static NANOMETRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 1_000_000_000.0,
};
pub static KILOMETRE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 0.001,
};
pub static MILE: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 0.000_621_371_192_237_333_9,
};
pub static FOOT: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 3.280_839_895_013_123,
};
pub static INCH: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 39.370_078_740_157_48,
};
pub static YARD: Unit = Unit {
    metric: Metric::Length,
    position: Position::AfterAmount,
    factor: 1.093_613_298_337_707_8,
};

pub static QM: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 1.0,
};
pub static QDM: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 100.0,
};
pub static QCM: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 10000.0,
};
pub static QMM: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 1_000_000.0,
};
pub static AR: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 0.01,
};
pub static HECTAR: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 0.0001,
};
pub static QKM: Unit = Unit {
    metric: Metric::Area,
    position: Position::AfterAmount,
    factor: 0.000_001,
};

pub static CUBICMETRE: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 1.0,
};
pub static BARREL: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 6.289_810_770_432_105,
};
pub static CUBICFOOT: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 35.314_666_721_488_59,
};
pub static LITRE: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 1000.0,
};
pub static GALLON: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 264.172_052_358_148_4,
};
pub static PINT: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 2_113.376_418_865_187,
};
pub static CUBICINCH: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 61_023.744_094_732_29,
};
pub static CUBICCENTIMETRE: Unit = Unit {
    metric: Metric::Volume,
    position: Position::AfterAmount,
    factor: 1_000_000.0,
};

pub static AH: Unit = Unit {
    metric: Metric::Charge,
    position: Position::AfterAmount,
    factor: 1.0,
};
pub static MILLIAH: Unit = Unit {
    metric: Metric::Charge,
    position: Position::AfterAmount,
    factor: 1000.0,
};
pub static KILOAH: Unit = Unit {
    metric: Metric::Charge,
    position: Position::AfterAmount,
    factor: 0.001,
};
pub static MEGAAH: Unit = Unit {
    metric: Metric::Charge,
    position: Position::AfterAmount,
    factor: 0.000_000_1,
};

pub static JOULE: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 1.0,
};
pub static KJOULE: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 0.001,
};
pub static WH: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 0.000_277_777_777_777_8,
};
pub static KWH: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 0.000_000_277_777_777_8,
};
pub static MEGAWH: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 0.000_000_000_277_777_8,
};
pub static GWH: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 0.000_000_000_000_277_8,
};
pub static MILLIWH: Unit = Unit {
    metric: Metric::Energy,
    position: Position::AfterAmount,
    factor: 0.277_777_777_777_777_8,
};

pub static EUR: Unit = Unit {
    metric: Metric::Currency,
    position: Position::Both,
    factor: 0.133_858_6,
};
pub static EUR_SUFFIX: Unit = Unit {
    metric: Metric::Currency,
    position: Position::AfterAmount,
    factor: 0.133_858_6,
};
pub static USD: Unit = Unit {
    metric: Metric::Currency,
    position: Position::Both,
    factor: 0.145_196_01,
};
pub static USD_SUFFIX: Unit = Unit {
    metric: Metric::Currency,
    position: Position::AfterAmount,
    factor: 0.145_196_01,
};
pub static HUF: Unit = Unit {
    metric: Metric::Currency,
    position: Position::Both,
    factor: 50.891_426,
};
pub static HUF_SUFFIX: Unit = Unit {
    metric: Metric::Currency,
    position: Position::AfterAmount,
    factor: 50.891_426,
};
pub static INR: Unit = Unit {
    metric: Metric::Currency,
    position: Position::Both,
    factor: 11.941_661,
};
pub static INR_SUFFIX: Unit = Unit {
    metric: Metric::Currency,
    position: Position::AfterAmount,
    factor: 11.941_661,
};
pub static GBP: Unit = Unit {
    metric: Metric::Currency,
    position: Position::Both,
    factor: 0.117_863_55,
};
pub static GBP_SUFFIX: Unit = Unit {
    metric: Metric::Currency,
    position: Position::AfterAmount,
    factor: 0.117_863_55,
};
pub static CNY: Unit = Unit {
    metric: Metric::Currency,
    position: Position::Both,
    factor: 1.0,
};
pub static CNY_SUFFIX: Unit = Unit {
    metric: Metric::Currency,
    position: Position::AfterAmount,
    factor: 1.0,
};
