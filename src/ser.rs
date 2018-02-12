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
