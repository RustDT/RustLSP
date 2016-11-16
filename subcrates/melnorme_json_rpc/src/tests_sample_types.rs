// sample serde types, taken from serde doc: 
// https://github.com/serde-rs/serde#deserialization-without-macros

#![cfg(test)]

use serde;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub enum PointField {
    X,
    Y,
}

pub fn new_sample_params(x: i32, y: i32) -> Point {
    Point { x : x, y : y }
}


impl serde::Deserialize for PointField {
    fn deserialize<D>(deserializer: &mut D) -> Result<PointField, D::Error>
        where D: serde::de::Deserializer
    {
        struct PointFieldVisitor;

        impl serde::de::Visitor for PointFieldVisitor {
            type Value = PointField;

            fn visit_str<E>(&mut self, value: &str) -> Result<PointField, E>
                where E: serde::de::Error
            {
                match value {
                    "x" => Ok(PointField::X),
                    "y" => Ok(PointField::Y),
                    _ => Err(serde::de::Error::custom("expected x or y")),
                }
            }
        }

        deserializer.deserialize(PointFieldVisitor)
    }
}

impl serde::Deserialize for Point {
    fn deserialize<D>(deserializer: &mut D) -> Result<Point, D::Error>
        where D: serde::de::Deserializer
    {
        static FIELDS: &'static [&'static str] = &["x", "y"];
        deserializer.deserialize_struct("Point", FIELDS, PointVisitor)
    }
}

struct PointVisitor;

impl serde::de::Visitor for PointVisitor {
    type Value = Point;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Point, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut x = None;
        let mut y = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(PointField::X) => { x = Some(try!(visitor.visit_value())); }
                Some(PointField::Y) => { y = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let x = match x {
            Some(x) => x,
            None => try!(visitor.missing_field("x")),
        };

        let y = match y {
            Some(y) => y,
            None => try!(visitor.missing_field("y")),
        };

        try!(visitor.end());

        Ok(Point{ x: x, y: y })
    }
}


impl serde::Serialize for Point {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
        let elem_count = 2;
        let mut state = try!(serializer.serialize_struct("Point", elem_count)); 
        {
            try!(serializer.serialize_struct_elt(&mut state, "x", &self.x));
            try!(serializer.serialize_struct_elt(&mut state, "y", &self.y));
        }
        serializer.serialize_struct_end(state)
    }
}
