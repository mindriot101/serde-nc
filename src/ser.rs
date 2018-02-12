use serde::ser::{self, Serialize};
use error::{Error, Result};

pub struct Serializer {
    output: String,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer);
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        unimplemented!();
    }

    fn serialize_i8(self, val: i8) -> Result<()> {
        self.serialize_i64(val as i64)
    }

    fn serialize_i16(self, val: i16) -> Result<()> {
        self.serialize_i64(val as i64)
    }

    fn serialize_i32(self, val: i32) -> Result<()> {
        self.serialize_i64(val as i64)
    }

    fn serialize_i64(self, val: i64) -> Result<()> {
        unimplemented!();
    }

    fn serialize_u8(self, val: u8) -> Result<()> {
        self.serialize_u64(val as i64)
    }

    fn serialize_u16(self, val: u16) -> Result<()> {
        self.serialize_u64(val as i64)
    }

    fn serialize_u32(self, val: u32) -> Result<()> {
        self.serialize_u64(val as i64)
    }

    fn serialize_u64(self, val: u64) -> Result<()> {
        unimplemented!();
    }

    fn serialize_f32(self, val: f32) -> Result<()> {
        self.serialize_f64(val as f64)
    }

    fn serialize_f64(self, val: f64) -> Result<()> {
        unimplemented!();
    }

    fn serialize_char(self, v: char) -> Result<()> {
        unimplemented!();
    }

    fn serialize_str(self, s: &str) -> Result<()> {
        unimplemented!();
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        unimplemented!();
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!();
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unimplemented!();
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!();
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        unimplemented!();
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<()> {
        unimplemented!();
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unimplemented!();
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
        {
            unimplemented!();
        }

    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where T: ?Sized + Serialize {
        unimplemented!();
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize {
    }

    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> Result<()>
    where
        T: Serialize
        {
            unimplemented!();
        }
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> Result<()>
    where
        T: Serialize
        {
            unimplemented!();
        }
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> Result<()>
    where
        T: Serialize
        {
            unimplemented!();
        }
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> Result<()>
    where
        T: Serialize
        {
            unimplemented!();
        }
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> Result<()>
    where
        T: Serialize
        {
            unimplemented!();
        }
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}
