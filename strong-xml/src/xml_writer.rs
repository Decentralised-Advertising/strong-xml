use std::io::Result;
use std::io::Write;
use std::iter::repeat;

use crate::xml_escape::xml_escape;

pub struct XmlWriter<W: Write> {
    pub inner: W,
    pub pretty: bool,
    pub level: usize,
}

impl<W: Write> XmlWriter<W> {
    pub fn new(inner: W, pretty: bool) -> Self {
        XmlWriter {
            inner,
            pretty,
            level: 0,
        }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    pub fn write_element_start(&mut self, tag: &str) -> Result<()> {
        if self.pretty {
            write!(self.inner, "{}", self.prefix())?;
        }
        write!(self.inner, "<{}", tag)
    }

    pub fn write_attribute(&mut self, key: &str, value: &str) -> Result<()> {
        write!(self.inner, r#" {}="{}""#, key, xml_escape(value))
    }

    pub fn write_text(&mut self, content: &str) -> Result<()> {
        write!(self.inner, "{}", xml_escape(content))
    }

    pub fn write_cdata_text(&mut self, content: &str) -> Result<()> {
        if self.pretty {
            write!(self.inner, "{}", self.prefix())?;
        }
        write!(self.inner, "<![CDATA[{}]]>", content)?;
        if self.pretty {
            write!(self.inner, "\n")?;
        }
        Ok(())
    }

    pub fn write_element_end_open(&mut self) -> Result<()> {
        write!(self.inner, ">")?;
        if self.pretty {
            self.level += 1;
            write!(self.inner, "\n")?;
        }
        Ok(())
    }

    pub fn write_flatten_text(&mut self, tag: &str, content: &str, is_cdata: bool) -> Result<()> {
        self.write_element_start(tag)?;
        self.write_element_end_open()?;
        if is_cdata {
            self.write_cdata_text(content)?;
        } else {
            self.write_text(content)?;
        }
        self.write_element_end_close(tag)?;
        Ok(())
    }

    pub fn write_element_end_close(&mut self, tag: &str) -> Result<()> {
        if self.pretty {
            self.level -= 1;
            write!(self.inner, "{}", self.prefix())?;
        }
        write!(self.inner, "</{}>", tag)?;
        if self.pretty {
            write!(self.inner, "\n")?;
        }
        Ok(())
    }

    pub fn write_element_end_empty(&mut self) -> Result<()> {
        write!(self.inner, "/>")?;
        if self.pretty {
            self.level += 1;
            write!(self.inner, "\n")?;
        }
        Ok(())
    }

    fn prefix(&self) -> String {
        repeat("    ").take(self.level).collect::<String>()
    }
}
