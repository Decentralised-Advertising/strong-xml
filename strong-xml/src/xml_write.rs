use std::io::Write;

use crate::{XmlResult, XmlWriter};

pub trait XmlWrite {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()>;

    fn to_string(&self) -> XmlResult<String> {
        let mut writer = XmlWriter::new(Vec::new(), false);

        self.to_writer(&mut writer)?;

        Ok(String::from_utf8(writer.inner)?)
    }

    fn to_string_pretty(&self) -> XmlResult<String> {
        let mut writer = XmlWriter::new(Vec::new(), true);

        self.to_writer(&mut writer)?;

        Ok(String::from_utf8(writer.inner)?)
    }
}
