#![feature(once_cell)]
#![deny(clippy::pedantic)]
#![allow(clippy::similar_names)]

use std::{collections::HashMap, sync::Arc};

use regex::{Regex, RegexBuilder};
use teloxide::prelude::*;
use text2num::{replace_numbers, Language};

#[derive(Debug, Clone)]
enum Metric {
    Weight,
    Length,
    Area,
    Volume,
    Charge,
    Energy,
    Currency,
}

#[derive(Debug, Clone, PartialEq)]
enum Position {
    BeforeAmount,
    AfterAmount,
    Both,
}

#[derive(Debug, Clone)]
struct Unit {
    metric: Metric,
    position: Position,
    factor: f64,
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
    fn in_davincis(&self, amount: f64) -> f64 {
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

#[allow(clippy::too_many_lines)]
fn get_units() -> HashMap<&'static str, Unit> {
    let mut map = HashMap::new();

    let kilogram = Unit {
        metric: Metric::Weight,
        position: Position::AfterAmount,
        factor: 1.0,
    };
    let gram = Unit {
        metric: Metric::Weight,
        position: Position::AfterAmount,
        factor: 1000.0,
    };
    let milligram = Unit {
        metric: Metric::Weight,
        position: Position::AfterAmount,
        factor: 1_000_000.0,
    };
    let microgram = Unit {
        metric: Metric::Weight,
        position: Position::AfterAmount,
        factor: 1_000_000_000.0,
    };
    let ton = Unit {
        metric: Metric::Weight,
        position: Position::AfterAmount,
        factor: 0.001,
    };

    let metre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 1.0,
    };
    let decimetre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 10.0,
    };
    let centimetre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 100.0,
    };
    let millimetre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 1000.0,
    };
    let micrometre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 1_000_000.0,
    };
    let nanometre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 1_000_000_000.0,
    };
    let kilometre = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 0.001,
    };
    let mile = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 0.000_621_371_192_237_333_9,
    };
    let foot = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 3.280_839_895_013_123,
    };
    let inch = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 39.370_078_740_157_48,
    };
    let yard = Unit {
        metric: Metric::Length,
        position: Position::AfterAmount,
        factor: 1.093_613_298_337_707_8,
    };

    let qm = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 1.0,
    };
    let qdm = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 100.0,
    };
    let qcm = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 10000.0,
    };
    let qmm = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 1_000_000.0,
    };
    let ar = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 0.01,
    };
    let hectar = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 0.0001,
    };
    let qkm = Unit {
        metric: Metric::Area,
        position: Position::AfterAmount,
        factor: 0.000_001,
    };

    let cubicmetre = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 1.0,
    };
    let barrel = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 6.289_810_770_432_105,
    };
    let cubicfoot = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 35.314_666_721_488_59,
    };
    let litre = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 1000.0,
    };
    let gallon = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 264.172_052_358_148_4,
    };
    let pint = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 2_113.376_418_865_187,
    };
    let cubicinch = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 61_023.744_094_732_29,
    };
    let cubiccentimetre = Unit {
        metric: Metric::Volume,
        position: Position::AfterAmount,
        factor: 1_000_000.0,
    };

    let ah = Unit {
        metric: Metric::Charge,
        position: Position::AfterAmount,
        factor: 1.0,
    };
    let milliah = Unit {
        metric: Metric::Charge,
        position: Position::AfterAmount,
        factor: 1000.0,
    };
    let kiloah = Unit {
        metric: Metric::Charge,
        position: Position::AfterAmount,
        factor: 0.001,
    };
    let megaah = Unit {
        metric: Metric::Charge,
        position: Position::AfterAmount,
        factor: 0.000_000_1,
    };

    let joule = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 1.0,
    };
    let kjoule = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 0.001,
    };
    let wh = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 0.000_277_777_777_777_8,
    };
    let kwh = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 0.000_000_277_777_777_8,
    };
    let megawh = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 0.000_000_000_277_777_8,
    };
    let gwh = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 0.000_000_000_000_277_8,
    };
    let milliwh = Unit {
        metric: Metric::Energy,
        position: Position::AfterAmount,
        factor: 0.277_777_777_777_777_8,
    };

    let eur = Unit {
        metric: Metric::Currency,
        position: Position::Both,
        factor: 0.133_858_6,
    };
    let eur_suffix = Unit {
        metric: Metric::Currency,
        position: Position::AfterAmount,
        factor: 0.133_858_6,
    };
    let usd = Unit {
        metric: Metric::Currency,
        position: Position::Both,
        factor: 0.145_196_01,
    };
    let usd_suffix = Unit {
        metric: Metric::Currency,
        position: Position::AfterAmount,
        factor: 0.145_196_01,
    };
    let huf = Unit {
        metric: Metric::Currency,
        position: Position::Both,
        factor: 50.891_426,
    };
    let huf_suffix = Unit {
        metric: Metric::Currency,
        position: Position::AfterAmount,
        factor: 50.891_426,
    };
    let inr = Unit {
        metric: Metric::Currency,
        position: Position::Both,
        factor: 11.941_661,
    };
    let inr_suffix = Unit {
        metric: Metric::Currency,
        position: Position::AfterAmount,
        factor: 11.941_661,
    };
    let gbp = Unit {
        metric: Metric::Currency,
        position: Position::Both,
        factor: 0.117_863_55,
    };
    let gbp_suffix = Unit {
        metric: Metric::Currency,
        position: Position::AfterAmount,
        factor: 0.117_863_55,
    };
    let cny = Unit {
        metric: Metric::Currency,
        position: Position::Both,
        factor: 1.0,
    };
    let cny_suffix = Unit {
        metric: Metric::Currency,
        position: Position::AfterAmount,
        factor: 1.0,
    };

    map.insert("kg", kilogram.clone());
    map.insert("kgs", kilogram.clone());
    map.insert("kilo", kilogram.clone());
    map.insert("kilogram", kilogram.clone());
    map.insert("kilos", kilogram.clone());
    map.insert("kilograms", kilogram);

    map.insert("g", gram.clone());
    map.insert("gram", gram.clone());
    map.insert("grams", gram);

    map.insert("mg", milligram.clone());
    map.insert("mgs", milligram.clone());
    map.insert("milligram", milligram.clone());
    map.insert("milligrams", milligram);

    map.insert("ug", microgram.clone());
    map.insert("μg", microgram.clone());
    map.insert("ugs", microgram.clone());
    map.insert("μgs", microgram.clone());
    map.insert("microgram", microgram.clone());
    map.insert("micrograms", microgram);

    map.insert("t", ton.clone());
    map.insert("ton", ton.clone());
    map.insert("tons", ton);

    map.insert("m", metre.clone());
    map.insert("meter", metre.clone());
    map.insert("meters", metre.clone());
    map.insert("metres", metre.clone());
    map.insert("metre", metre);

    map.insert("dm", decimetre.clone());
    map.insert("decimeter", decimetre.clone());
    map.insert("decimeters", decimetre.clone());
    map.insert("decimetres", decimetre.clone());
    map.insert("decimetre", decimetre);

    map.insert("cm", centimetre.clone());
    map.insert("centimeter", centimetre.clone());
    map.insert("centimeters", centimetre.clone());
    map.insert("centimetres", centimetre.clone());
    map.insert("centimetre", centimetre);

    map.insert("mm", millimetre.clone());
    map.insert("millimeter", millimetre.clone());
    map.insert("millimeters", millimetre.clone());
    map.insert("millimetres", millimetre.clone());
    map.insert("millimetre", millimetre);

    map.insert("um", micrometre.clone());
    map.insert("μm", micrometre.clone());
    map.insert("micrometer", micrometre.clone());
    map.insert("micrometers", micrometre.clone());
    map.insert("micrometres", micrometre.clone());
    map.insert("micrometre", micrometre);

    map.insert("nm", nanometre.clone());
    map.insert("nanometer", nanometre.clone());
    map.insert("nanometers", nanometre.clone());
    map.insert("nanometres", nanometre.clone());
    map.insert("nanometre", nanometre);

    map.insert("km", kilometre.clone());
    map.insert("kilometer", kilometre.clone());
    map.insert("kilometers", kilometre.clone());
    map.insert("kilometres", kilometre.clone());
    map.insert("kilometre", kilometre);

    map.insert("mile", mile.clone());
    map.insert("miles", mile);

    map.insert("foot", foot.clone());
    map.insert("feet", foot);

    map.insert("in", inch.clone());
    map.insert("inch", inch.clone());
    map.insert("inches", inch);

    map.insert("yd", yard.clone());
    map.insert("yard", yard.clone());
    map.insert("yards", yard);

    map.insert("m²", qm.clone());
    map.insert("qm", qm.clone());
    map.insert("sqm", qm.clone());
    map.insert("m2", qm.clone());
    map.insert("squaremeter", qm.clone());
    map.insert("squaremeters", qm.clone());
    map.insert("squaremetre", qm.clone());
    map.insert("squaremetres", qm.clone());
    map.insert("square meter", qm.clone());
    map.insert("square meters", qm.clone());
    map.insert("square metre", qm.clone());
    map.insert("square metres", qm.clone());

    map.insert("dm²", qdm.clone());
    map.insert("qdm", qdm.clone());
    map.insert("sqdm", qdm.clone());
    map.insert("dm2", qdm.clone());
    map.insert("squaredecimeter", qdm.clone());
    map.insert("squaredecimeters", qdm.clone());
    map.insert("squaredecimetre", qdm.clone());
    map.insert("squaredecimetres", qdm.clone());
    map.insert("square decimeter", qdm.clone());
    map.insert("square decimeters", qdm.clone());
    map.insert("square decimetre", qdm.clone());
    map.insert("square decimetres", qdm);

    map.insert("cm²", qcm.clone());
    map.insert("qcm", qcm.clone());
    map.insert("sqcm", qcm.clone());
    map.insert("cm2", qcm.clone());
    map.insert("squarecentimeter", qcm.clone());
    map.insert("squarecentimeters", qcm.clone());
    map.insert("squarecentimetre", qcm.clone());
    map.insert("squarecentimetres", qcm.clone());
    map.insert("square centimeter", qcm.clone());
    map.insert("square centimeters", qcm.clone());
    map.insert("square centimetre", qcm.clone());
    map.insert("square centimetres", qcm);

    map.insert("mm²", qmm.clone());
    map.insert("qmm", qmm.clone());
    map.insert("sqmm", qmm.clone());
    map.insert("mm2", qmm.clone());
    map.insert("squaremillimeter", qmm.clone());
    map.insert("squaremillimeters", qmm.clone());
    map.insert("squaremillimetre", qmm.clone());
    map.insert("squaremillimetres", qmm.clone());
    map.insert("square millimeter", qmm.clone());
    map.insert("square millimeters", qmm.clone());
    map.insert("square millimetre", qmm.clone());
    map.insert("square millimetres", qmm);

    map.insert("ar", ar.clone());
    map.insert("ars", ar);

    map.insert("ha", hectar.clone());
    map.insert("has", hectar.clone());
    map.insert("hectar", hectar.clone());
    map.insert("hectars", hectar);

    map.insert("km²", qkm.clone());
    map.insert("qkm", qm);
    map.insert("sqkm", qkm.clone());
    map.insert("km2", qkm.clone());
    map.insert("squarekilometer", qkm.clone());
    map.insert("squarekilometers", qkm.clone());
    map.insert("squarekilometre", qkm.clone());
    map.insert("squarekilometres", qkm.clone());
    map.insert("square kilometer", qkm.clone());
    map.insert("square kilometers", qkm.clone());
    map.insert("square kilometre", qkm.clone());
    map.insert("square kilometres", qkm);

    map.insert("m³", cubicmetre.clone());
    map.insert("m3", cubicmetre.clone());
    map.insert("cubicmeter", cubicmetre.clone());
    map.insert("cubicmeters", cubicmetre.clone());
    map.insert("cubicmetre", cubicmetre.clone());
    map.insert("cubicmetres", cubicmetre.clone());
    map.insert("cubic meter", cubicmetre.clone());
    map.insert("cubic meters", cubicmetre.clone());
    map.insert("cubic metre", cubicmetre.clone());
    map.insert("cubic metres", cubicmetre);

    map.insert("barrel", barrel.clone());
    map.insert("barrels", barrel);

    map.insert("cubicfoot", cubicfoot.clone());
    map.insert("cubic foot", cubicfoot.clone());
    map.insert("cubicfeet", cubicfoot.clone());
    map.insert("cubic feet", cubicfoot);

    map.insert("dm³", litre.clone());
    map.insert("dm3", litre.clone());
    map.insert("cubicdecimeter", litre.clone());
    map.insert("cubicdecimetre", litre.clone());
    map.insert("cubicdecimeters", litre.clone());
    map.insert("cubicdecimetres", litre.clone());
    map.insert("cubic decimeter", litre.clone());
    map.insert("cubic decimetre", litre.clone());
    map.insert("cubic decimeters", litre.clone());
    map.insert("cubic decimetres", litre.clone());
    map.insert("liter", litre.clone());
    map.insert("litre", litre.clone());
    map.insert("liters", litre.clone());
    map.insert("litres", litre);

    map.insert("gallon", gallon.clone());
    map.insert("gallons", gallon.clone());
    map.insert("gal", gallon);

    map.insert("pint", pint.clone());
    map.insert("pints", pint);

    map.insert("cubic inch", cubicinch.clone());
    map.insert("cubic inches", cubicinch.clone());
    map.insert("cubicinch", cubicinch.clone());
    map.insert("cubicinches", cubicinch);

    map.insert("cm³", cubiccentimetre.clone());
    map.insert("cm3", cubiccentimetre.clone());
    map.insert("cubiccentimeter", cubiccentimetre.clone());
    map.insert("cubiccentimeters", cubiccentimetre.clone());
    map.insert("cubiccentimetre", cubiccentimetre.clone());
    map.insert("cubiccentimetres", cubiccentimetre.clone());
    map.insert("cubic centimeter", cubiccentimetre.clone());
    map.insert("cubic centimeters", cubiccentimetre.clone());
    map.insert("cubic centimetre", cubiccentimetre.clone());
    map.insert("cubic centimetres", cubiccentimetre);

    map.insert("ah", ah.clone());
    map.insert("amperehour", ah.clone());
    map.insert("amperehours", ah.clone());
    map.insert("ampere hour", ah.clone());
    map.insert("ampere hours", ah);

    map.insert("mah", milliah.clone());
    map.insert("milliamperehour", milliah.clone());
    map.insert("milliamperehours", milliah.clone());
    map.insert("milliampere hour", milliah.clone());
    map.insert("milliampere hours", milliah);

    map.insert("kah", kiloah.clone());
    map.insert("kiloamperehour", kiloah.clone());
    map.insert("kiloamperehours", kiloah.clone());
    map.insert("kiloampere hour", kiloah.clone());
    map.insert("kiloampere hours", kiloah);

    map.insert("megaamperehour", megaah.clone());
    map.insert("megaamperehours", megaah.clone());
    map.insert("megaampere hour", megaah.clone());
    map.insert("megaampere hours", megaah);

    map.insert("j", joule.clone());
    map.insert("joule", joule.clone());
    map.insert("joules", joule);

    map.insert("kj", kjoule.clone());
    map.insert("kilojoule", kjoule.clone());
    map.insert("kilojoules", kjoule.clone());
    map.insert("kilo joule", kjoule.clone());
    map.insert("kilo joules", kjoule);

    map.insert("wh", wh.clone());
    map.insert("watthour", wh.clone());
    map.insert("watthours", wh.clone());
    map.insert("watt hour", wh.clone());
    map.insert("watt hours", wh);

    map.insert("kwh", kwh.clone());
    map.insert("kilowatthour", kwh.clone());
    map.insert("kilowatthours", kwh.clone());
    map.insert("kilowatt hour", kwh.clone());
    map.insert("kilowatt hours", kwh);

    map.insert("mwh", megawh.clone());
    map.insert("megawatthour", megawh.clone());
    map.insert("megawatthours", megawh.clone());
    map.insert("megawatt hour", megawh.clone());
    map.insert("megawatt hours", megawh);

    map.insert("gwh", gwh.clone());
    map.insert("gigawatthour", gwh.clone());
    map.insert("gigawatthours", gwh.clone());
    map.insert("gigawatt hour", gwh.clone());
    map.insert("gigawatt hours", gwh);

    map.insert("milliwatthour", milliwh.clone());
    map.insert("milliwatthours", milliwh.clone());
    map.insert("milliwatt hour", milliwh.clone());
    map.insert("milliwatt hours", milliwh);

    map.insert("€", eur.clone());
    map.insert("eur", eur);
    map.insert("euro", eur_suffix.clone());
    map.insert("euros", eur_suffix);

    map.insert("$", usd.clone());
    map.insert("usd", usd);
    map.insert("dollar", usd_suffix.clone());
    map.insert("dollars", usd_suffix);

    map.insert("ft", huf.clone());
    map.insert("huf", huf);
    map.insert("fts", huf_suffix.clone());
    map.insert("hufs", huf_suffix.clone());
    map.insert("forint", huf_suffix.clone());
    map.insert("forints", huf_suffix);

    map.insert("₹", inr.clone());
    map.insert("inr", inr.clone());
    map.insert("rs", inr);
    map.insert("rupee", inr_suffix.clone());
    map.insert("rupees", inr_suffix);

    map.insert("£", gbp.clone());
    map.insert("gbp", gbp);
    map.insert("pound", gbp_suffix.clone());
    map.insert("pounds", gbp_suffix.clone());
    map.insert("pound sterling", gbp_suffix.clone());
    map.insert("pounds sterling", gbp_suffix);

    map.insert("¥", cny.clone());
    map.insert("cny", cny.clone());
    map.insert("rmb", cny);
    map.insert("yuan", cny_suffix.clone());
    map.insert("chinese yuan", cny_suffix);

    map
}

fn normalize(input: &str) -> String {
    let english = Language::english();
    replace_numbers(input, &english, 0.0)
}

async fn run() {
    println!("Building regexes");

    let units = get_units();

    let mut units_sorted: Vec<(&'static str, &Unit)> = units.iter().map(|(a, b)| (*a, b)).collect();
    units_sorted.sort_unstable_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

    // Regex 1: Match a unit before the amount
    let mut unit_before_regex = String::from("(?:^|\\b)(?P<unit>");

    for (idx, (key, _)) in units_sorted
        .iter()
        .filter(|(_, unit)| {
            unit.position == Position::BeforeAmount || unit.position == Position::Both
        })
        .enumerate()
    {
        if idx != 0 {
            unit_before_regex.push('|');
        }
        unit_before_regex.push_str(&regex::escape(key));
    }
    unit_before_regex.push_str(")\\s?(?P<amount>\\d+(?:\\.\\d+)?)(?P<kilo>k\\s)?");

    // Regex 2: Match a unit after the amount
    let mut unit_after_regex =
        String::from("(?P<amount>\\d+(?:\\.\\d+)?)(?P<kilo>k\\s)?\\s*(?P<unit>");

    for (idx, (key, _)) in units_sorted
        .into_iter()
        .filter(|(_, unit)| {
            unit.position == Position::AfterAmount || unit.position == Position::Both
        })
        .enumerate()
    {
        if idx != 0 {
            unit_after_regex.push('|');
        }
        unit_after_regex.push_str(&regex::escape(key));
    }
    unit_after_regex.push_str(")(?:\\b|\\s|$)");

    let unit_before_regex = RegexBuilder::new(&unit_before_regex)
        .case_insensitive(true)
        .multi_line(true)
        .build()
        .unwrap();
    let unit_after_regex = RegexBuilder::new(&unit_after_regex)
        .case_insensitive(true)
        .multi_line(true)
        .build()
        .unwrap();
    let regexes = [unit_before_regex, unit_after_regex];

    println!("Starting playground metrics bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message().endpoint(
        |bot: Bot,
         regexes: Arc<[Regex; 2]>,
         units: Arc<HashMap<&'static str, Unit>>,
         msg: Message| async move {
            if let Some(msg_text) = msg.text() {
                let normalized = normalize(msg_text);
                let mut text = normalized.clone();

                for regex in regexes.iter() {
                    while let Some(cap) = regex.captures(&text) {
                        let mut amount: f64 =
                            lexical_core::parse(cap["amount"].as_bytes()).unwrap();

                        if cap.name("kilo").is_some() {
                            amount *= 1000.0;
                        }

                        let unit_value = cap["unit"].to_lowercase();
                        let unit = units.get(unit_value.as_str()).unwrap();

                        let davincis = (unit.in_davincis(amount) * 100.0).round() / 100.0;

                        let old = cap.get(0).unwrap();

                        let new = if (davincis - 1.0).abs() < f64::EPSILON {
                            format!("{davincis} davinci")
                        } else {
                            format!("{davincis} davincis")
                        };

                        // old.range() is not really correct because we might have trailing
                        // whitespace
                        let range = old.start()..old.start() + old.as_str().trim_end().len();

                        text.replace_range(range, &new);
                    }
                }

                if normalized != text {
                    bot.send_message(msg.chat.id, text)
                        .reply_to_message_id(msg.id)
                        .await?;
                }
            }

            respond(())
        },
    );

    Dispatcher::builder(bot, handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![Arc::new(regexes), Arc::new(units)])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run());
}
