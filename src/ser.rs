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
    value.serialize(&mut serializer).unwrap();
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

    fn serialize_bool(self, _v: bool) -> Result<()> {
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

    fn serialize_i64(self, _val: i64) -> Result<()> {
        unimplemented!();
    }

    fn serialize_u8(self, val: u8) -> Result<()> {
        self.serialize_u64(val as u64)
    }

    fn serialize_u16(self, val: u16) -> Result<()> {
        self.serialize_u64(val as u64)
    }

    fn serialize_u32(self, val: u32) -> Result<()> {
        self.serialize_u64(val as u64)
    }

    fn serialize_u64(self, _val: u64) -> Result<()> {
        unimplemented!();
    }

    fn serialize_f32(self, val: f32) -> Result<()> {
        self.serialize_f64(val as f64)
    }

    fn serialize_f64(self, _val: f64) -> Result<()> {
        unimplemented!();
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!();
    }

    fn serialize_str(self, _s: &str) -> Result<()> {
        unimplemented!();
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!();
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!();
    }

    fn serialize_some<T>(self, _v: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!();
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!();
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!();
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!();
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!();
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!();
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!();
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!();
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!();
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
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

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
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

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
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

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
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

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
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

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!();
    }
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}
