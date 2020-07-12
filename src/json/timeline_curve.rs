use serde::de::{value::SeqAccessDeserializer, Error as SerdeError, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone)]
pub enum TimelineCurve {
    CurveLinear,
    CurveStepped,
    CurveBezier(Vec<f32>),
}

impl<'a> Deserialize<'a> for TimelineCurve {
    fn deserialize<D>(deserializer: D) -> Result<TimelineCurve, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_any(TimelineCurveVisitor)
    }
}

struct TimelineCurveVisitor;

impl<'a> Visitor<'a> for TimelineCurveVisitor {
    type Value = TimelineCurve;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "array of floats or one of (linear, stepped)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        match value {
            "linear" => Ok(TimelineCurve::CurveLinear),
            "stepped" => Ok(TimelineCurve::CurveStepped),
            _ => Err(SerdeError::custom(format!(
                "Timeline curve must be either linear, stepped or an array"
            ))),
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        self.visit_str(value.as_ref())
    }

    fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'a>,
    {
        let array = Deserialize::deserialize(SeqAccessDeserializer::new(visitor))?;

        Ok(TimelineCurve::CurveBezier(array))
    }
}
