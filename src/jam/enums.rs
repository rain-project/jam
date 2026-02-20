use crate::{
    Jam,
    extensions::{ReadExt, WriteExt},
};
use std::io::{self, Read, Write};

impl<T> Jam for Option<T>
where
    T: Jam,
{
    fn read_from<R>(reader: &mut R) -> io::Result<Option<T>>
    where
        R: Read,
    {
        let variant = reader.read_u8()?;

        let value = match variant {
            0 => None,
            1 => Some(T::read_from(reader)?),

            _ => return Err(io::ErrorKind::InvalidData.into()),
        };

        Ok(value)
    }

    fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        match self {
            None => {
                writer.write_u8(0)?;
            }

            Some(value) => {
                writer.write_u8(1)?;
                value.write_into(writer)?;
            }
        }

        Ok(())
    }
}

impl<T, E> Jam for Result<T, E>
where
    T: Jam,
    E: Jam,
{
    fn read_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read,
    {
        let variant = reader.read_u8()?;

        let value = match variant {
            0 => Ok(T::read_from(reader)?),
            1 => Err(E::read_from(reader)?),

            _ => return Err(io::ErrorKind::InvalidData.into()),
        };

        Ok(value)
    }

    fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        match self {
            Ok(value) => {
                writer.write_u8(0)?;
                value.write_into(writer)?;
            }

            Err(error) => {
                writer.write_u8(1)?;
                error.write_into(writer)?;
            }
        }

        Ok(())
    }
}
