use std::collections::BTreeMap;
use std::io::{Cursor, Read};

use anyhow::{Context, Result};

const LPP_DIGITAL_INPUT: u8 = 0; // 1 byte
const LPP_DIGITAL_OUTPUT: u8 = 1; // 1 byte
const LPP_ANALOG_INPUT: u8 = 2; // 2 bytes, 0.01 signed
const LPP_ANALOG_OUTPUT: u8 = 3; // 2 bytes, 0.01 signed
const LPP_GENERIC_SENSOR: u8 = 100; // 4 bytes, unsigned
const LPP_ILLUMINANCE_SENSOR: u8 = 101; // 2 bytes, 1 lux unsigned
const LPP_PRESENCE_SENSOR: u8 = 102; // 1 byte, bool
const LPP_TEMPERATURE_SENSOR: u8 = 103; // 2 bytes, 0.1°C signed
const LPP_HUMIDITY_SENSOR: u8 = 104; // 1 byte, 0.5% unsigned
const LPP_ACCELEROMETER: u8 = 113; // 2 bytes per axis, 0.001G
const LPP_BAROMETER: u8 = 115; // 2 bytes 0.1hPa unsigned
const LPP_VOLTAGE: u8 = 116; // 2 bytes 0.01V unsigned
const LPP_CURRENT: u8 = 117; // 2 bytes 0.001A unsigned
const LPP_FREQUENCY: u8 = 118; // 4 bytes 1Hz unsigned
const LPP_PERCENTAGE: u8 = 120; // 1 byte 1-100% unsigned
const LPP_ALTITUDE: u8 = 121; // 2 byte 1m signed
const LPP_CONCENTRATION: u8 = 125; // 2 bytes, 1 ppm unsigned
const LPP_POWER: u8 = 128; // 2 byte, 1W, unsigned
const LPP_DISTANCE: u8 = 130; // 4 byte, 0.001m, unsigned
const LPP_ENERGY: u8 = 131; // 4 byte, 0.001kWh, unsigned
const LPP_DIRECTION: u8 = 132; // 2 bytes, 1deg, unsigned
const LPP_UNIXTIME: u8 = 133; // 4 bytes, unsigned
const LPP_GYROMETER: u8 = 134; // 2 bytes per axis, 0.01 °/s
const LPP_COLOUR: u8 = 135; // 1 byte per RGB Color
const LPP_GPS_LOCATION: u8 = 136; // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter
const LPP_SWITCH: u8 = 142; // 1 byte, 0/1

pub fn decode(b: &[u8]) -> Result<pbjson_types::Struct> {
    let lpp = CayenneLpp::from_slice(b).context("Decode Cayenne LPP payload")?;
    Ok(lpp.to_struct())
}

pub fn encode(obj: &prost_types::Struct) -> Result<Vec<u8>> {
    let lpp = CayenneLpp::from_struct(obj).context("Encode Cayenne LPP payload")?;
    Ok(lpp.to_vec())
}

struct Accelerometer {
    x: f64,
    y: f64,
    z: f64,
}

struct Gyrometer {
    x: f64,
    y: f64,
    z: f64,
}

struct GpsLocation {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default)]
struct CayenneLpp {
    digital_input: BTreeMap<u8, u8>,
    digital_output: BTreeMap<u8, u8>,
    analog_input: BTreeMap<u8, f64>,
    analog_output: BTreeMap<u8, f64>,
    generic_sensor: BTreeMap<u8, u32>,
    illuminance_sensor: BTreeMap<u8, u16>,
    presence_sensor: BTreeMap<u8, u8>,
    temperature_sensor: BTreeMap<u8, f64>,
    humidity_sensor: BTreeMap<u8, f64>,
    accelerometer: BTreeMap<u8, Accelerometer>,
    barometer: BTreeMap<u8, f64>,
    voltage: BTreeMap<u8, f64>,
    current: BTreeMap<u8, f64>,
    frequency: BTreeMap<u8, u32>,
    percentage: BTreeMap<u8, u8>,
    altitude: BTreeMap<u8, i16>,
    concentration: BTreeMap<u8, u16>,
    power: BTreeMap<u8, u16>,
    distance: BTreeMap<u8, f64>,
    energy: BTreeMap<u8, f64>,
    direction: BTreeMap<u8, u16>,
    unixtime: BTreeMap<u8, u32>,
    gyrometer: BTreeMap<u8, Gyrometer>,
    colour: BTreeMap<u8, Colour>,
    gps_location: BTreeMap<u8, GpsLocation>,
    switch: BTreeMap<u8, u8>,
}

impl CayenneLpp {
    fn from_slice(b: &[u8]) -> Result<Self> {
        let mut lpp: CayenneLpp = Default::default();
        let mut cur = Cursor::new(b);

        loop {
            let mut buf: Vec<u8> = vec![0; 2];
            if cur.read_exact(&mut buf).is_err() {
                break;
            };

            match buf[1] {
                LPP_DIGITAL_INPUT => lpp.set_digital_input(buf[0], &mut cur)?,
                LPP_DIGITAL_OUTPUT => lpp.set_digital_output(buf[0], &mut cur)?,
                LPP_ANALOG_INPUT => lpp.set_analog_input(buf[0], &mut cur)?,
                LPP_ANALOG_OUTPUT => lpp.set_analog_output(buf[0], &mut cur)?,
                LPP_GENERIC_SENSOR => lpp.set_generic_sensor(buf[0], &mut cur)?,
                LPP_ILLUMINANCE_SENSOR => lpp.set_illuminance_sensor(buf[0], &mut cur)?,
                LPP_PRESENCE_SENSOR => lpp.set_presence_sensor(buf[0], &mut cur)?,
                LPP_TEMPERATURE_SENSOR => lpp.set_temperature_sensor(buf[0], &mut cur)?,
                LPP_HUMIDITY_SENSOR => lpp.set_humidity_sensor(buf[0], &mut cur)?,
                LPP_ACCELEROMETER => lpp.set_accelerometer(buf[0], &mut cur)?,
                LPP_BAROMETER => lpp.set_barometer(buf[0], &mut cur)?,
                LPP_VOLTAGE => lpp.set_voltage(buf[0], &mut cur)?,
                LPP_CURRENT => lpp.set_current(buf[0], &mut cur)?,
                LPP_FREQUENCY => lpp.set_frequency(buf[0], &mut cur)?,
                LPP_PERCENTAGE => lpp.set_percentage(buf[0], &mut cur)?,
                LPP_ALTITUDE => lpp.set_altitude(buf[0], &mut cur)?,
                LPP_CONCENTRATION => lpp.set_concentration(buf[0], &mut cur)?,
                LPP_POWER => lpp.set_power(buf[0], &mut cur)?,
                LPP_DISTANCE => lpp.set_distance(buf[0], &mut cur)?,
                LPP_ENERGY => lpp.set_energy(buf[0], &mut cur)?,
                LPP_DIRECTION => lpp.set_direction(buf[0], &mut cur)?,
                LPP_UNIXTIME => lpp.set_unixtime(buf[0], &mut cur)?,
                LPP_GYROMETER => lpp.set_gyrometer(buf[0], &mut cur)?,
                LPP_COLOUR => lpp.set_colour(buf[0], &mut cur)?,
                LPP_GPS_LOCATION => lpp.set_gps_location(buf[0], &mut cur)?,
                LPP_SWITCH => lpp.set_switch(buf[0], &mut cur)?,
                _ => {
                    return Err(anyhow!("Invalid data type: {}", buf[1]));
                }
            }
        }

        Ok(lpp)
    }

    fn from_struct(s: &prost_types::Struct) -> Result<Self> {
        let mut lpp: CayenneLpp = Default::default();

        for (k, v) in &s.fields {
            match k.as_ref() {
                "digitalInput" => lpp
                    .set_digital_input_from_value(v)
                    .context("digitalInput")?,
                "digitalOutput" => lpp
                    .set_digital_output_from_value(v)
                    .context("digitalOutput")?,
                "analogInput" => lpp.set_analog_input_from_value(v).context("analogInput")?,
                "analogOutput" => lpp
                    .set_analog_output_from_value(v)
                    .context("analogOutput")?,
                "genericSensor" => lpp
                    .set_generic_sensor_from_value(v)
                    .context("genericSensor")?,
                "illuminanceSensor" => lpp
                    .set_illuminance_sensor_from_value(v)
                    .context("illuminanceSensor")?,
                "presenceSensor" => lpp
                    .set_presence_sensor_from_value(v)
                    .context("presenceSensor")?,
                "temperatureSensor" => lpp
                    .set_temperature_sensor_from_value(v)
                    .context("temperatureSensor")?,
                "humiditySensor" => lpp
                    .set_humidity_sensor_from_value(v)
                    .context("humiditySensor")?,
                "accelerometer" => lpp
                    .set_accelerometer_from_value(v)
                    .context("accelerometer")?,
                "barometer" => lpp.set_barometer_from_value(v).context("barometer")?,
                "voltage" => lpp.set_voltage_from_value(v).context("voltage")?,
                "current" => lpp.set_current_from_value(v).context("current")?,
                "frequency" => lpp.set_frequency_from_value(v).context("frequency")?,
                "percentage" => lpp.set_percentage_from_value(v).context("percentage")?,
                "altitude" => lpp.set_altitude_from_value(v).context("altitude")?,
                "concentration" => lpp
                    .set_concentration_from_value(v)
                    .context("concentration")?,
                "power" => lpp.set_power_from_value(v).context("power")?,
                "distance" => lpp.set_distance_from_value(v).context("distance")?,
                "energy" => lpp.set_energy_from_value(v).context("energy")?,
                "direction" => lpp.set_direction_from_value(v).context("direction")?,
                "unixtime" => lpp.set_unixtime_from_value(v).context("unixtime")?,
                "gyrometer" => lpp.set_gyrometer_from_value(v).context("gyrometer")?,
                "colour" => lpp.set_colour_from_value(v).context("colour")?,
                "gpsLocation" => lpp.set_gps_location_from_value(v).context("gpsLocation")?,
                "switch" => lpp.set_switch_from_value(v).context("switch")?,
                _ => {
                    return Err(anyhow!("Unexpected key '{}' in payload", k));
                }
            }
        }

        Ok(lpp)
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        // digital input
        for (k, v) in &self.digital_input {
            out.extend([*k, LPP_DIGITAL_INPUT]);
            out.push(*v);
        }

        // digital output
        for (k, v) in &self.digital_output {
            out.extend([*k, LPP_DIGITAL_OUTPUT]);
            out.push(*v);
        }

        // analog input
        for (k, v) in &self.analog_input {
            out.extend([*k, LPP_ANALOG_INPUT]);

            let val = (*v * 100.0) as i16;
            out.extend(val.to_be_bytes());
        }

        // analog output
        for (k, v) in &self.analog_output {
            out.extend([*k, LPP_ANALOG_OUTPUT]);

            let val = (*v * 100.0) as i16;
            out.extend(val.to_be_bytes());
        }

        // generic sensor
        for (k, v) in &self.generic_sensor {
            out.extend([*k, LPP_GENERIC_SENSOR]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // illuminance sensor
        for (k, v) in &self.illuminance_sensor {
            out.extend([*k, LPP_ILLUMINANCE_SENSOR]);
            out.extend(v.to_be_bytes());
        }

        // presence sensor
        for (k, v) in &self.presence_sensor {
            out.extend([*k, LPP_PRESENCE_SENSOR]);
            out.push(*v);
        }

        // temperature sensor
        for (k, v) in &self.temperature_sensor {
            out.extend([*k, LPP_TEMPERATURE_SENSOR]);

            let val = (*v * 10.0) as i16;
            out.extend(val.to_be_bytes());
        }

        // humidity sensor
        for (k, v) in &self.humidity_sensor {
            out.extend([*k, LPP_HUMIDITY_SENSOR]);

            let val = (*v * 2.0) as u8;
            out.push(val);
        }

        // accelerometer
        for (k, v) in &self.accelerometer {
            out.extend([*k, LPP_ACCELEROMETER]);

            let x = (v.x * 1000.0) as i16;
            let y = (v.y * 1000.0) as i16;
            let z = (v.z * 1000.0) as i16;
            out.extend(x.to_be_bytes());
            out.extend(y.to_be_bytes());
            out.extend(z.to_be_bytes());
        }

        // barometer
        for (k, v) in &self.barometer {
            out.extend([*k, LPP_BAROMETER]);

            let val = (*v * 10.0) as u16;
            out.extend(val.to_be_bytes());
        }

        // voltage
        for (k, v) in &self.voltage {
            out.extend([*k, LPP_VOLTAGE]);

            let val = (*v * 100.0) as u16;
            out.extend(val.to_be_bytes());
        }

        // current
        for (k, v) in &self.current {
            out.extend([*k, LPP_CURRENT]);

            let val = (*v * 1000.0) as u16;
            out.extend(val.to_be_bytes());
        }

        // frequency
        for (k, v) in &self.frequency {
            out.extend([*k, LPP_FREQUENCY]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // percentage
        for (k, v) in &self.percentage {
            out.extend([*k, LPP_PERCENTAGE]);

            let val = *v;
            out.push(val);
        }

        // altitude
        for (k, v) in &self.altitude {
            out.extend([*k, LPP_ALTITUDE]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // concentration
        for (k, v) in &self.concentration {
            out.extend([*k, LPP_CONCENTRATION]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // power
        for (k, v) in &self.power {
            out.extend([*k, LPP_POWER]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // distance
        for (k, v) in &self.distance {
            out.extend([*k, LPP_DISTANCE]);

            let val = (*v * 1000.0) as u32;
            out.extend(val.to_be_bytes());
        }

        // energy
        for (k, v) in &self.energy {
            out.extend([*k, LPP_ENERGY]);

            let val = (*v * 1000.0) as u32;
            out.extend(val.to_be_bytes());
        }

        // direction
        for (k, v) in &self.direction {
            out.extend([*k, LPP_DIRECTION]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // unixtime
        for (k, v) in &self.unixtime {
            out.extend([*k, LPP_UNIXTIME]);

            let val = *v;
            out.extend(val.to_be_bytes());
        }

        // gyrometer
        for (k, v) in &self.gyrometer {
            out.extend([*k, LPP_GYROMETER]);

            let x = (v.x * 100.0) as i16;
            let y = (v.y * 100.0) as i16;
            let z = (v.z * 100.0) as i16;
            out.extend(x.to_be_bytes());
            out.extend(y.to_be_bytes());
            out.extend(z.to_be_bytes());
        }

        // colour
        for (k, v) in &self.colour {
            out.extend([*k, LPP_COLOUR]);

            let r = v.r;
            let g = v.g;
            let b = v.b;
            out.push(r);
            out.push(g);
            out.push(b);
        }

        // gps location
        for (k, v) in &self.gps_location {
            out.extend([*k, LPP_GPS_LOCATION]);

            let lat = (v.latitude * 10000.0) as i32;
            let lon = (v.longitude * 10000.0) as i32;
            let alt = (v.altitude * 100.0) as i32;

            out.extend(&lat.to_be_bytes()[1..]);
            out.extend(&lon.to_be_bytes()[1..]);
            out.extend(&alt.to_be_bytes()[1..]);
        }

        // switch
        for (k, v) in &self.switch {
            out.extend([*k, LPP_SWITCH]);
            out.push(*v);
        }
        out
    }

    fn to_struct(&self) -> pbjson_types::Struct {
        let mut out: pbjson_types::Struct = Default::default();

        if !self.digital_input.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.digital_input {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "digitalInput".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.digital_output.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.digital_output {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "digitalOutput".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.analog_input.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.analog_input {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "analogInput".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.analog_output.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.analog_output {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "analogOutput".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.generic_sensor.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.generic_sensor {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "genericSensor".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.illuminance_sensor.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.illuminance_sensor {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "illuminanceSensor".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.presence_sensor.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.presence_sensor {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "presenceSensor".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.temperature_sensor.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.temperature_sensor {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "temperatureSensor".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.humidity_sensor.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.humidity_sensor {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "humiditySensor".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.accelerometer.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.accelerometer {
                let mut item: pbjson_types::Struct = Default::default();
                item.fields.insert(
                    "x".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.x)),
                    },
                );
                item.fields.insert(
                    "y".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.y)),
                    },
                );
                item.fields.insert(
                    "z".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.z)),
                    },
                );

                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(item)),
                    },
                );
            }
            out.fields.insert(
                "accelerometer".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.barometer.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.barometer {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "barometer".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.voltage.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.voltage {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "voltage".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.current.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.current {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "current".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.frequency.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.frequency {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "frequency".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.percentage.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.percentage {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "percentage".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.altitude.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.altitude {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "altitude".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.concentration.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.concentration {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "concentration".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.power.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.power {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "power".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.distance.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.distance {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "distance".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.energy.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.energy {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v)),
                    },
                );
            }
            out.fields.insert(
                "energy".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.direction.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.direction {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "direction".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.unixtime.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.unixtime {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "unixtime".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.gyrometer.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.gyrometer {
                let mut item: pbjson_types::Struct = Default::default();
                item.fields.insert(
                    "x".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.x)),
                    },
                );
                item.fields.insert(
                    "y".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.y)),
                    },
                );
                item.fields.insert(
                    "z".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.z)),
                    },
                );

                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(item)),
                    },
                );
            }
            out.fields.insert(
                "gyrometer".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.colour.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.colour {
                let mut item: pbjson_types::Struct = Default::default();
                item.fields.insert(
                    "r".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.r as f64)),
                    },
                );
                item.fields.insert(
                    "g".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.g as f64)),
                    },
                );
                item.fields.insert(
                    "b".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.b as f64)),
                    },
                );

                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(item)),
                    },
                );
            }
            out.fields.insert(
                "colour".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.gps_location.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.gps_location {
                let mut item: pbjson_types::Struct = Default::default();
                item.fields.insert(
                    "latitude".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.latitude)),
                    },
                );
                item.fields.insert(
                    "longitude".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.longitude)),
                    },
                );
                item.fields.insert(
                    "altitude".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(v.altitude)),
                    },
                );

                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(item)),
                    },
                );
            }
            out.fields.insert(
                "gpsLocation".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }

        if !self.switch.is_empty() {
            let mut val: pbjson_types::Struct = Default::default();
            for (k, v) in &self.switch {
                val.fields.insert(
                    format!("{}", k),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(*v as f64)),
                    },
                );
            }
            out.fields.insert(
                "switch".to_string(),
                pbjson_types::Value {
                    kind: Some(pbjson_types::value::Kind::StructValue(val)),
                },
            );
        }
        out
    }

    fn set_digital_input(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf)?;
        self.digital_input.insert(channel, buf[0]);
        Ok(())
    }

    fn set_digital_input_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.digital_input.insert(c, *v as u8);
                }
            }
        }

        Ok(())
    }

    fn set_digital_output(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf)?;
        self.digital_output.insert(channel, buf[0]);
        Ok(())
    }

    fn set_digital_output_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.digital_output.insert(c, *v as u8);
                }
            }
        }

        Ok(())
    }

    fn set_analog_input(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = i16::from_be_bytes(buf);
        self.analog_input.insert(channel, (val as f64) / 100.0);
        Ok(())
    }

    fn set_analog_input_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.analog_input.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_analog_output(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = i16::from_be_bytes(buf);
        self.analog_output.insert(channel, (val as f64) / 100.0);
        Ok(())
    }

    fn set_analog_output_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.analog_output.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_generic_sensor(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        cur.read_exact(&mut buf)?;
        let val = u32::from_be_bytes(buf);
        self.generic_sensor.insert(channel, val);
        Ok(())
    }

    fn set_generic_sensor_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.generic_sensor.insert(c, *v as u32);
                }
            }
        }

        Ok(())
    }

    fn set_illuminance_sensor(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.illuminance_sensor.insert(channel, val);
        Ok(())
    }

    fn set_illuminance_sensor_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.illuminance_sensor.insert(c, *v as u16);
                }
            }
        }

        Ok(())
    }

    fn set_presence_sensor(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf)?;
        self.presence_sensor.insert(channel, buf[0]);
        Ok(())
    }

    fn set_presence_sensor_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.presence_sensor.insert(c, *v as u8);
                }
            }
        }

        Ok(())
    }

    fn set_temperature_sensor(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = i16::from_be_bytes(buf);
        self.temperature_sensor.insert(channel, (val as f64) / 10.0);
        Ok(())
    }

    fn set_temperature_sensor_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.temperature_sensor.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_humidity_sensor(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf)?;
        self.humidity_sensor.insert(channel, (buf[0] as f64) / 2.0);
        Ok(())
    }

    fn set_humidity_sensor_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.humidity_sensor.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_accelerometer(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf_x: [u8; 2] = [0; 2];
        let mut buf_y: [u8; 2] = [0; 2];
        let mut buf_z: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf_x)?;
        cur.read_exact(&mut buf_y)?;
        cur.read_exact(&mut buf_z)?;
        self.accelerometer.insert(
            channel,
            Accelerometer {
                x: (i16::from_be_bytes(buf_x) as f64) / 1000.0,
                y: (i16::from_be_bytes(buf_y) as f64) / 1000.0,
                z: (i16::from_be_bytes(buf_z) as f64) / 1000.0,
            },
        );
        Ok(())
    }

    fn set_accelerometer_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                let mut item = Accelerometer {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };

                if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
                    let x = s
                        .fields
                        .get("x")
                        .ok_or_else(|| anyhow!("x field is missing"))?;
                    let y = s
                        .fields
                        .get("y")
                        .ok_or_else(|| anyhow!("y field is missing"))?;
                    let z = s
                        .fields
                        .get("z")
                        .ok_or_else(|| anyhow!("z field is missing"))?;

                    if let Some(prost_types::value::Kind::NumberValue(v)) = &x.kind {
                        item.x = *v;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &y.kind {
                        item.y = *v;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &z.kind {
                        item.z = *v;
                    }
                }

                self.accelerometer.insert(c, item);
            }
        }

        Ok(())
    }

    fn set_barometer(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.barometer.insert(channel, (val as f64) / 10.0);
        Ok(())
    }

    fn set_barometer_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.barometer.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_voltage(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.voltage.insert(channel, (val as f64) / 100.0);
        Ok(())
    }

    fn set_voltage_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.voltage.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_current(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.current.insert(channel, (val as f64) / 1000.0);
        Ok(())
    }

    fn set_current_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.current.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_frequency(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        cur.read_exact(&mut buf)?;
        let val = u32::from_be_bytes(buf);
        self.frequency.insert(channel, val);
        Ok(())
    }

    fn set_frequency_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.frequency.insert(c, *v as u32);
                }
            }
        }

        Ok(())
    }

    fn set_percentage(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf)?;
        let val = u8::from_be_bytes(buf);
        self.percentage.insert(channel, val);
        Ok(())
    }

    fn set_percentage_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.percentage.insert(c, *v as u8);
                }
            }
        }

        Ok(())
    }

    fn set_altitude(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = i16::from_be_bytes(buf);
        self.altitude.insert(channel, val);
        Ok(())
    }

    fn set_altitude_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.altitude.insert(c, *v as i16);
                }
            }
        }

        Ok(())
    }

    fn set_concentration(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.concentration.insert(channel, val);
        Ok(())
    }

    fn set_concentration_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.concentration.insert(c, *v as u16);
                }
            }
        }

        Ok(())
    }

    fn set_power(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.power.insert(channel, val);
        Ok(())
    }

    fn set_power_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.power.insert(c, *v as u16);
                }
            }
        }

        Ok(())
    }

    fn set_distance(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        cur.read_exact(&mut buf)?;
        let val = u32::from_be_bytes(buf);
        self.distance.insert(channel, (val as f64) / 1000.0);
        Ok(())
    }

    fn set_distance_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.distance.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_energy(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        cur.read_exact(&mut buf)?;
        let val = u32::from_be_bytes(buf);
        self.energy.insert(channel, (val as f64) / 1000.0);
        Ok(())
    }

    fn set_energy_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.energy.insert(c, *v);
                }
            }
        }

        Ok(())
    }

    fn set_direction(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf)?;
        let val = u16::from_be_bytes(buf);
        self.direction.insert(channel, val);
        Ok(())
    }

    fn set_direction_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.direction.insert(c, *v as u16);
                }
            }
        }

        Ok(())
    }

    fn set_unixtime(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        cur.read_exact(&mut buf)?;
        let val = u32::from_be_bytes(buf);
        self.unixtime.insert(channel, val);
        Ok(())
    }

    fn set_unixtime_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.unixtime.insert(c, *v as u32);
                }
            }
        }

        Ok(())
    }

    fn set_gyrometer(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf_x: [u8; 2] = [0; 2];
        let mut buf_y: [u8; 2] = [0; 2];
        let mut buf_z: [u8; 2] = [0; 2];
        cur.read_exact(&mut buf_x)?;
        cur.read_exact(&mut buf_y)?;
        cur.read_exact(&mut buf_z)?;
        self.gyrometer.insert(
            channel,
            Gyrometer {
                x: (i16::from_be_bytes(buf_x) as f64) / 100.0,
                y: (i16::from_be_bytes(buf_y) as f64) / 100.0,
                z: (i16::from_be_bytes(buf_z) as f64) / 100.0,
            },
        );
        Ok(())
    }

    fn set_gyrometer_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                let mut item = Gyrometer {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };

                if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
                    let x = s
                        .fields
                        .get("x")
                        .ok_or_else(|| anyhow!("x field is missing"))?;
                    let y = s
                        .fields
                        .get("y")
                        .ok_or_else(|| anyhow!("y field is missing"))?;
                    let z = s
                        .fields
                        .get("z")
                        .ok_or_else(|| anyhow!("z field is missing"))?;

                    if let Some(prost_types::value::Kind::NumberValue(v)) = &x.kind {
                        item.x = *v;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &y.kind {
                        item.y = *v;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &z.kind {
                        item.z = *v;
                    }
                }

                self.gyrometer.insert(c, item);
            }
        }

        Ok(())
    }

    fn set_colour(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf_r: [u8; 1] = [0; 1];
        let mut buf_g: [u8; 1] = [0; 1];
        let mut buf_b: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf_r)?;
        cur.read_exact(&mut buf_g)?;
        cur.read_exact(&mut buf_b)?;
        self.colour.insert(
            channel,
            Colour {
                r: (buf_r[0]),
                g: (buf_g[0]),
                b: (buf_b[0]),
            },
        );
        Ok(())
    }

    fn set_colour_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                let mut item = Colour { r: 0, g: 0, b: 0 };

                if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
                    let r = s
                        .fields
                        .get("r")
                        .ok_or_else(|| anyhow!("r field is missing"))?;
                    let g = s
                        .fields
                        .get("g")
                        .ok_or_else(|| anyhow!("g field is missing"))?;
                    let b = s
                        .fields
                        .get("b")
                        .ok_or_else(|| anyhow!("b field is missing"))?;

                    if let Some(prost_types::value::Kind::NumberValue(v)) = &r.kind {
                        item.r = *v as u8;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &g.kind {
                        item.g = *v as u8;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &b.kind {
                        item.b = *v as u8;
                    }
                }

                self.colour.insert(c, item);
            }
        }

        Ok(())
    }

    fn set_gps_location(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf_lat: [u8; 3] = [0; 3];
        let mut buf_lon: [u8; 3] = [0; 3];
        let mut buf_alt: [u8; 3] = [0; 3];
        cur.read_exact(&mut buf_lat)?;
        cur.read_exact(&mut buf_lon)?;
        cur.read_exact(&mut buf_alt)?;
        self.gps_location.insert(
            channel,
            GpsLocation {
                latitude: ((i32::from_be_bytes([buf_lat[0], buf_lat[1], buf_lat[2], 0]) >> 8)
                    as f64)
                    / 10000.0,
                longitude: ((i32::from_be_bytes([buf_lon[0], buf_lon[1], buf_lon[2], 0]) >> 8)
                    as f64)
                    / 10000.0,
                altitude: ((i32::from_be_bytes([buf_alt[0], buf_alt[1], buf_alt[2], 0]) >> 8)
                    as f64)
                    / 100.0,
            },
        );
        Ok(())
    }

    fn set_gps_location_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                let mut item = GpsLocation {
                    latitude: 0.0,
                    longitude: 0.0,
                    altitude: 0.0,
                };

                if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
                    let lat = s
                        .fields
                        .get("latitude")
                        .ok_or_else(|| anyhow!("latitude field is missing"))?;
                    let lon = s
                        .fields
                        .get("longitude")
                        .ok_or_else(|| anyhow!("longitude field is missing"))?;
                    let alt = s
                        .fields
                        .get("altitude")
                        .ok_or_else(|| anyhow!("altitude field is missing"))?;

                    if let Some(prost_types::value::Kind::NumberValue(v)) = &lat.kind {
                        item.latitude = *v;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &lon.kind {
                        item.longitude = *v;
                    }
                    if let Some(prost_types::value::Kind::NumberValue(v)) = &alt.kind {
                        item.altitude = *v;
                    }

                    self.gps_location.insert(c, item);
                }
            }
        }

        Ok(())
    }

    fn set_switch(&mut self, channel: u8, cur: &mut Cursor<&[u8]>) -> Result<()> {
        let mut buf: [u8; 1] = [0; 1];
        cur.read_exact(&mut buf)?;
        self.switch.insert(channel, buf[0]);
        Ok(())
    }

    fn set_switch_from_value(&mut self, v: &prost_types::Value) -> Result<()> {
        if let Some(prost_types::value::Kind::StructValue(s)) = &v.kind {
            for (k, v) in &s.fields {
                let c: u8 = k.parse()?;
                if let Some(prost_types::value::Kind::NumberValue(v)) = &v.kind {
                    self.switch.insert(c, *v as u8);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_lpp() {
        let b: Vec<u8> = vec![
            3, 0, 100, 5, 0, 210, // digital input
            3, 1, 100, 5, 1, 210, // digital output
            3, 2, 0, 10, 5, 2, 3, 232, // analog input
            3, 3, 0, 10, 5, 3, 3, 232, // analog output
            3, 100, 0, 0, 0, 1, 5, 100, 255, 254, 253, 252, // generic sensors
            3, 101, 0, 10, 5, 101, 3, 232, // illuminance sensors
            3, 102, 5, 5, 102, 3, // presence sensors
            3, 103, 1, 16, 5, 103, 0, 255, // temperature sensors
            3, 104, 41, 5, 104, 150, // humidity sensors
            3, 113, 0, 1, 0, 2, 0, 3, 5, 113, 3, 234, 7, 211, 11, 187, // accelerometers
            3, 115, 4, 31, 5, 115, 9, 196, // barometers
            3, 116, 0, 1, 5, 116, 255, 254, //voltage
            3, 117, 0, 2, 5, 117, 255, 253, //current
            3, 118, 0, 0, 0, 1, 5, 118, 255, 254, 253, 251, //frequency
            3, 120, 0, 5, 120, 100, //percentage
            3, 121, 0, 1, 5, 121, 255, 251, //altitude
            3, 125, 0, 7, 5, 125, 255, 254, //concentration
            3, 128, 0, 8, 5, 128, 254, 253, //power
            3, 130, 0, 0, 1, 16, 5, 130, 1, 2, 3, 4, // distance
            3, 131, 0, 0, 0, 12, 5, 131, 255, 254, 253, 254, // energy
            3, 132, 0, 9, 5, 132, 255, 252, //direction
            3, 133, 0, 0, 0, 13, 5, 133, 255, 254, 253, 253, // unixtime
            3, 134, 0, 1, 0, 2, 0, 3, 5, 134, 3, 233, 7, 210, 11, 187, // gyrometers
            3, 135, 100, 150, 200, 5, 135, 250, 190, 0, // rgb-colour
            1, 136, 6, 118, 95, 242, 150, 10, 0, 3, 232, // gps location
            3, 142, 0, 5, 142, 1, //switch
        ];
        let prost_struct = prost_types::Struct {
            fields: [
                (
                    "digitalInput".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(100.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(210.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "digitalOutput".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(100.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(210.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "analogInput".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.1)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(10.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "analogOutput".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.1)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(10.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "genericSensor".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(1.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(4294901244.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "illuminanceSensor".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(10.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(1000.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "presenceSensor".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(5.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(3.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "temperatureSensor".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(27.2)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(25.5)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "humiditySensor".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(20.5)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(75.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "accelerometer".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StructValue(
                                            prost_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.001,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.002,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.003,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StructValue(
                                            prost_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                1.002,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                2.003,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                3.003,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "barometer".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(105.5)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(250.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "voltage".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.01)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(655.34)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "current".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.002)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(65.533)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "frequency".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(1.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(4294901243.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "percentage".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(100.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "altitude".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(1.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(-5.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "concentration".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(7.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(65534.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "power".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(8.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(65277.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "distance".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.272)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(16909.060)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "energy".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.012)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(4294901.246)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "direction".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(9.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(65532.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "unixtime".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(13.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(4294901245.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "gyrometer".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StructValue(
                                            prost_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.01,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.02,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.03,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StructValue(
                                            prost_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                10.01,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                20.02,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                30.03,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "colour".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StructValue(
                                            prost_types::Struct {
                                                fields: [(
                                                    "r".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                100.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "g".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                150.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "b".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                200.0,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::StructValue(
                                            prost_types::Struct {
                                                fields: [(
                                                    "r".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                250.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "g".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                190.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "b".to_string(),
                                                    prost_types::Value {
                                                        kind: Some(
                                                            prost_types::value::Kind::NumberValue(
                                                                0.0,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "gpsLocation".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "1".to_string(),
                                    prost_types::Value {
                                        kind: Some(
                                              prost_types::value::Kind::StructValue(prost_types::Struct{
                                                  fields: [
                                                      (
                                                          "latitude".to_string(),
                                                          prost_types::Value {
                                                              kind: Some(prost_types::value::Kind::NumberValue(42.3519)),
                                                          },
                                                      ),
                                                      (
                                                          "longitude".to_string(),
                                                          prost_types::Value {
                                                              kind: Some(prost_types::value::Kind::NumberValue(-87.9094)),
                                                          },
                                                      ),
                                                      (
                                                          "altitude".to_string(),
                                                          prost_types::Value {
                                                              kind: Some(prost_types::value::Kind::NumberValue(10.0)),
                                                          },
                                                      ),
                                                  ].iter().cloned().collect(),
                                              }),
                                          ),
                                    },
                                ),
                            ].iter().cloned().collect(),
                        })),
                    },
                ),
                (
                    "switch".to_string(),
                    prost_types::Value {
                        kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(0.0)),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    prost_types::Value {
                                        kind: Some(prost_types::value::Kind::NumberValue(1.0)),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        let pbjson_struct = pbjson_types::Struct {
            fields: [
                (
                    "digitalInput".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                100.0,
                                            )),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                210.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "digitalOutput".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                100.0,
                                            )),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                210.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "analogInput".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.1)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                10.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "analogOutput".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.1)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                10.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "genericSensor".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(1.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                4294901244.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "illuminanceSensor".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                10.0,
                                            )),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                1000.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "presenceSensor".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(5.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(3.0)),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "temperatureSensor".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                27.2,
                                            )),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                25.5,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "humiditySensor".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                20.5,
                                            )),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                                75.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "accelerometer".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(pbjson_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StructValue(
                                            pbjson_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.001,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.002,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.003,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StructValue(
                                            pbjson_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                1.002,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                2.003,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                3.003,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "barometer".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(105.5)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              250.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "voltage".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.01)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              655.34,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "current".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.002)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              65.533,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "frequency".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(1.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              4294901243.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "percentage".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              100.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "altitude".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(1.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              -5.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "concentration".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(7.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              65534.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "power".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(8.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              65277.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "distance".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.272)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              16909.060,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "energy".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.012)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              4294901.246,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "direction".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(9.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              65532.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "unixtime".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(13.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              4294901245.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "gyrometer".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(pbjson_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StructValue(
                                            pbjson_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.01,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.02,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.03,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StructValue(
                                            pbjson_types::Struct {
                                                fields: [(
                                                    "x".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                10.01,
                                                            ),
                                                        ),
                                                    }),(
                                                    "y".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                20.02,
                                                            ),
                                                        ),
                                                    }),(
                                                    "z".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                30.03,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "colour".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(pbjson_types::Struct {
                            fields: [
                                (
                                    "3".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StructValue(
                                            pbjson_types::Struct {
                                                fields: [(
                                                    "r".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                100.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "g".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                150.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "b".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                200.0,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                                (
                                    "5".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StructValue(
                                            pbjson_types::Struct {
                                                fields: [(
                                                    "r".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                250.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "g".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                190.0,
                                                            ),
                                                        ),
                                                    }),(
                                                    "b".to_string(),
                                                    pbjson_types::Value {
                                                        kind: Some(
                                                            pbjson_types::value::Kind::NumberValue(
                                                                0.0,
                                                            ),
                                                        ),
                                                    },
                                                )]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            },
                                        )),
                                    },
                                ),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        })),
                    },
                ),
                (
                    "gpsLocation".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(pbjson_types::Struct {
                            fields: [
                                (
                                    "1".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(
                                              pbjson_types::value::Kind::StructValue(pbjson_types::Struct{
                                                  fields: [
                                                      (
                                                          "latitude".to_string(),
                                                          pbjson_types::Value {
                                                              kind: Some(pbjson_types::value::Kind::NumberValue(42.3519)),
                                                          },
                                                      ),
                                                      (
                                                          "longitude".to_string(),
                                                          pbjson_types::Value {
                                                              kind: Some(pbjson_types::value::Kind::NumberValue(-87.9094)),
                                                          },
                                                      ),
                                                      (
                                                          "altitude".to_string(),
                                                          pbjson_types::Value {
                                                              kind: Some(pbjson_types::value::Kind::NumberValue(10.0)),
                                                          },
                                                      ),
                                                  ].iter().cloned().collect(),
                                              }),
                                          ),
                                    },
                                ),
                            ].iter().cloned().collect(),
                        })),
                    },
                ),
                (
                    "switch".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [
                                    (
                                        "3".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(0.0)),
                                        },
                                    ),
                                    (
                                        "5".to_string(),
                                        pbjson_types::Value {
                                            kind: Some(pbjson_types::value::Kind::NumberValue(
                                              1.0,
                                            )),
                                        },
                                    ),
                                ]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        let lpp_decode = decode(&b).unwrap();
        assert_eq!(pbjson_struct, lpp_decode);

        let b_encode = encode(&prost_struct).unwrap();
        assert_eq!(b, b_encode);
    }
}
