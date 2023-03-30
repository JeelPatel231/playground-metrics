#![deny(clippy::pedantic)]
#![allow(clippy::similar_names)]

use std::{collections::HashMap, sync::Arc};

use regex::{Regex, RegexBuilder};
use teloxide::prelude::*;
use text2num::{replace_numbers, Language};
mod models;
mod units;

use models::{Metric, Position, Unit};
use units::*;
type UnitMap = HashMap<&'static str, &'static Unit>;

fn get_units() -> UnitMap {
    HashMap::from([
        ("kg", &KILOGRAM),
        ("kgs", &KILOGRAM),
        ("kilo", &KILOGRAM),
        ("kilogram", &KILOGRAM),
        ("kilos", &KILOGRAM),
        ("kilograms", &KILOGRAM),
        ("g", &GRAM),
        ("gram", &GRAM),
        ("grams", &GRAM),
        ("mg", &MILLIGRAM),
        ("mgs", &MILLIGRAM),
        ("milligram", &MILLIGRAM),
        ("milligrams", &MILLIGRAM),
        ("ug", &MICROGRAM),
        ("μg", &MICROGRAM),
        ("ugs", &MICROGRAM),
        ("μgs", &MICROGRAM),
        ("microgram", &MICROGRAM),
        ("micrograms", &MICROGRAM),
        ("t", &TON),
        ("ton", &TON),
        ("tons", &TON),
        ("m", &METRE),
        ("meter", &METRE),
        ("meters", &METRE),
        ("metres", &METRE),
        ("metre", &METRE),
        ("dm", &DECIMETRE),
        ("decimeter", &DECIMETRE),
        ("decimeters", &DECIMETRE),
        ("decimetres", &DECIMETRE),
        ("decimetre", &DECIMETRE),
        ("cm", &CENTIMETRE),
        ("centimeter", &CENTIMETRE),
        ("centimeters", &CENTIMETRE),
        ("centimetres", &CENTIMETRE),
        ("centimetre", &CENTIMETRE),
        ("mm", &MILLIMETRE),
        ("millimeter", &MILLIMETRE),
        ("millimeters", &MILLIMETRE),
        ("millimetres", &MILLIMETRE),
        ("millimetre", &MILLIMETRE),
        ("um", &MICROMETRE),
        ("μm", &MICROMETRE),
        ("micrometer", &MICROMETRE),
        ("micrometers", &MICROMETRE),
        ("micrometres", &MICROMETRE),
        ("micrometre", &MICROMETRE),
        ("nm", &NANOMETRE),
        ("nanometer", &NANOMETRE),
        ("nanometers", &NANOMETRE),
        ("nanometres", &NANOMETRE),
        ("nanometre", &NANOMETRE),
        ("km", &KILOMETRE),
        ("kilometer", &KILOMETRE),
        ("kilometers", &KILOMETRE),
        ("kilometres", &KILOMETRE),
        ("kilometre", &KILOMETRE),
        ("mile", &MILE),
        ("miles", &MILE),
        ("foot", &FOOT),
        ("feet", &FOOT),
        ("in", &INCH),
        ("inch", &INCH),
        ("inches", &INCH),
        ("yd", &YARD),
        ("yard", &YARD),
        ("yards", &YARD),
        ("m²", &QM),
        ("qm", &QM),
        ("sqm", &QM),
        ("m2", &QM),
        ("squaremeter", &QM),
        ("squaremeters", &QM),
        ("squaremetre", &QM),
        ("squaremetres", &QM),
        ("square meter", &QM),
        ("square meters", &QM),
        ("square metre", &QM),
        ("square metres", &QM),
        ("dm²", &QDM),
        ("qdm", &QDM),
        ("sqdm", &QDM),
        ("dm2", &QDM),
        ("squaredecimeter", &QDM),
        ("squaredecimeters", &QDM),
        ("squaredecimetre", &QDM),
        ("squaredecimetres", &QDM),
        ("square decimeter", &QDM),
        ("square decimeters", &QDM),
        ("square decimetre", &QDM),
        ("square decimetres", &QDM),
        ("cm²", &QCM),
        ("qcm", &QCM),
        ("sqcm", &QCM),
        ("cm2", &QCM),
        ("squarecentimeter", &QCM),
        ("squarecentimeters", &QCM),
        ("squarecentimetre", &QCM),
        ("squarecentimetres", &QCM),
        ("square centimeter", &QCM),
        ("square centimeters", &QCM),
        ("square centimetre", &QCM),
        ("square centimetres", &QCM),
        ("mm²", &QMM),
        ("qmm", &QMM),
        ("sqmm", &QMM),
        ("mm2", &QMM),
        ("squaremillimeter", &QMM),
        ("squaremillimeters", &QMM),
        ("squaremillimetre", &QMM),
        ("squaremillimetres", &QMM),
        ("square millimeter", &QMM),
        ("square millimeters", &QMM),
        ("square millimetre", &QMM),
        ("square millimetres", &QMM),
        ("ar", &AR),
        ("ars", &AR),
        ("ha", &HECTAR),
        ("has", &HECTAR),
        ("hectar", &HECTAR),
        ("hectars", &HECTAR),
        ("km²", &QKM),
        ("qkm", &QM),
        ("sqkm", &QKM),
        ("km2", &QKM),
        ("squarekilometer", &QKM),
        ("squarekilometers", &QKM),
        ("squarekilometre", &QKM),
        ("squarekilometres", &QKM),
        ("square kilometer", &QKM),
        ("square kilometers", &QKM),
        ("square kilometre", &QKM),
        ("square kilometres", &QKM),
        ("m³", &CUBICMETRE),
        ("m3", &CUBICMETRE),
        ("cubicmeter", &CUBICMETRE),
        ("cubicmeters", &CUBICMETRE),
        ("cubicmetre", &CUBICMETRE),
        ("cubicmetres", &CUBICMETRE),
        ("cubic meter", &CUBICMETRE),
        ("cubic meters", &CUBICMETRE),
        ("cubic metre", &CUBICMETRE),
        ("cubic metres", &CUBICMETRE),
        ("barrel", &BARREL),
        ("barrels", &BARREL),
        ("cubicfoot", &CUBICFOOT),
        ("cubic foot", &CUBICFOOT),
        ("cubicfeet", &CUBICFOOT),
        ("cubic feet", &CUBICFOOT),
        ("dm³", &LITRE),
        ("dm3", &LITRE),
        ("cubicdecimeter", &LITRE),
        ("cubicdecimetre", &LITRE),
        ("cubicdecimeters", &LITRE),
        ("cubicdecimetres", &LITRE),
        ("cubic decimeter", &LITRE),
        ("cubic decimetre", &LITRE),
        ("cubic decimeters", &LITRE),
        ("cubic decimetres", &LITRE),
        ("liter", &LITRE),
        ("litre", &LITRE),
        ("liters", &LITRE),
        ("litres", &LITRE),
        ("gallon", &GALLON),
        ("gallons", &GALLON),
        ("gal", &GALLON),
        ("pint", &PINT),
        ("pints", &PINT),
        ("cubic inch", &CUBICINCH),
        ("cubic inches", &CUBICINCH),
        ("cubicinch", &CUBICINCH),
        ("cubicinches", &CUBICINCH),
        ("cm³", &CUBICCENTIMETRE),
        ("cm3", &CUBICCENTIMETRE),
        ("cubiccentimeter", &CUBICCENTIMETRE),
        ("cubiccentimeters", &CUBICCENTIMETRE),
        ("cubiccentimetre", &CUBICCENTIMETRE),
        ("cubiccentimetres", &CUBICCENTIMETRE),
        ("cubic centimeter", &CUBICCENTIMETRE),
        ("cubic centimeters", &CUBICCENTIMETRE),
        ("cubic centimetre", &CUBICCENTIMETRE),
        ("cubic centimetres", &CUBICCENTIMETRE),
        ("ah", &AH),
        ("amperehour", &AH),
        ("amperehours", &AH),
        ("ampere hour", &AH),
        ("ampere hours", &AH),
        ("mah", &MILLIAH),
        ("milliamperehour", &MILLIAH),
        ("milliamperehours", &MILLIAH),
        ("milliampere hour", &MILLIAH),
        ("milliampere hours", &MILLIAH),
        ("kah", &KILOAH),
        ("kiloamperehour", &KILOAH),
        ("kiloamperehours", &KILOAH),
        ("kiloampere hour", &KILOAH),
        ("kiloampere hours", &KILOAH),
        ("megaamperehour", &MEGAAH),
        ("megaamperehours", &MEGAAH),
        ("megaampere hour", &MEGAAH),
        ("megaampere hours", &MEGAAH),
        ("j", &JOULE),
        ("joule", &JOULE),
        ("joules", &JOULE),
        ("kj", &KJOULE),
        ("kilojoule", &KJOULE),
        ("kilojoules", &KJOULE),
        ("kilo joule", &KJOULE),
        ("kilo joules", &KJOULE),
        ("wh", &WH),
        ("watthour", &WH),
        ("watthours", &WH),
        ("watt hour", &WH),
        ("watt hours", &WH),
        ("kwh", &KWH),
        ("kilowatthour", &KWH),
        ("kilowatthours", &KWH),
        ("kilowatt hour", &KWH),
        ("kilowatt hours", &KWH),
        ("mwh", &MEGAWH),
        ("megawatthour", &MEGAWH),
        ("megawatthours", &MEGAWH),
        ("megawatt hour", &MEGAWH),
        ("megawatt hours", &MEGAWH),
        ("gwh", &GWH),
        ("gigawatthour", &GWH),
        ("gigawatthours", &GWH),
        ("gigawatt hour", &GWH),
        ("gigawatt hours", &GWH),
        ("milliwatthour", &MILLIWH),
        ("milliwatthours", &MILLIWH),
        ("milliwatt hour", &MILLIWH),
        ("milliwatt hours", &MILLIWH),
        ("€", &EUR),
        ("eur", &EUR),
        ("euro", &EUR_SUFFIX),
        ("euros", &EUR_SUFFIX),
        ("$", &USD),
        ("usd", &USD),
        ("dollar", &USD_SUFFIX),
        ("dollars", &USD_SUFFIX),
        ("ft", &HUF),
        ("huf", &HUF),
        ("fts", &HUF_SUFFIX),
        ("hufs", &HUF_SUFFIX),
        ("forint", &HUF_SUFFIX),
        ("forints", &HUF_SUFFIX),
        ("₹", &INR),
        ("inr", &INR),
        ("rs", &INR),
        ("rupee", &INR_SUFFIX),
        ("rupees", &INR_SUFFIX),
        ("£", &GBP),
        ("gbp", &GBP),
        ("pound", &GBP_SUFFIX),
        ("pounds", &GBP_SUFFIX),
        ("pound sterling", &GBP_SUFFIX),
        ("pounds sterling", &GBP_SUFFIX),
        ("¥", &CNY),
        ("cny", &CNY),
        ("rmb", &CNY),
        ("yuan", &CNY_SUFFIX),
        ("chinese yuan", &CNY_SUFFIX),
    ])
}

fn normalize(input: &str) -> String {
    let english = Language::english();
    replace_numbers(input, &english, 0.0)
}

async fn run() {
    println!("Building regexes");

    let units = get_units();

    let mut units_sorted: Vec<(&'static str, &Unit)> =
        units.iter().map(|(a, b)| (*a, *b)).collect();
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
        |bot: Bot, regexes: Arc<[Regex; 2]>, units: Arc<UnitMap>, msg: Message| async move {
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
