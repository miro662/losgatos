use core::str;

use super::DeviceTreeError;

#[derive(Debug)]
pub(super) struct DeviceTreeTokenizer<'a> {
    pub(super) structure: &'a [u8],
    pub(super) strings: &'a [u8],
    pub(super) position: usize,
}

impl<'a> DeviceTreeTokenizer<'a> {
    const START_NODE_TOKEN: u32 = 0x1;
    const END_NODE_TOKEN: u32 = 0x2;
    const PARAM_TOKEN: u32 = 0x3;
    const NOP_NODE_TOKEN: u32 = 0x4;
    const END_TOKEN: u32 = 0x9;

    pub fn next_param(&mut self) -> Result<Option<(&str, &[u8])>, DeviceTreeError>{
        self.skip_nop()?;
        if self.seek_u32()? == Self::PARAM_TOKEN {
            let _ = self.next_u32()?; // move by param token
            let len = self.next_u32()? as usize;
            let nameoff = self.next_u32()? as usize;

            if self.position + len > self.structure.len() {
                return Err(DeviceTreeError::OutOfStructureBounds)
            }

            let name = Self::next_str(&self.strings[nameoff..])?;
            let value = &self.structure[self.position..(self.position + len)];

            self.proceed(len);
            Ok(Some((name, value)))
        } else {
            Ok(None)
        }
    }

    pub fn next_node(&mut self) -> Result<Option<&str>, DeviceTreeError> {
        self.skip_nop()?;
        if self.seek_u32()? == Self::START_NODE_TOKEN {
            let _ = self.next_u32();
            Ok(Some(self.next_str_local()?))
        } else {
            Ok(None)
        }
    }

    pub fn next_node_end(&mut self) -> Result<Option<()>, DeviceTreeError> {
        self.skip_nop()?;
        if self.seek_u32()? == Self::END_NODE_TOKEN {
            let _ = self.next_u32();
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    pub fn is_eof(&mut self) -> Result<bool, DeviceTreeError> {
        self.skip_nop();
        Ok(self.seek_u32()? == Self::END_TOKEN)
    }

    fn next_str_local(&mut self) -> Result<&str, DeviceTreeError> {
        self.skip_nop();
        let s = Self::next_str(&self.structure[self.position..])?;
        self.proceed(s.bytes().len() + 1);
        Ok(s)
    }

    fn next_str(slice: &[u8]) -> Result<&str, DeviceTreeError> {
        let Some(str_len) = slice.iter().position(|&c| c == 0) else {
            return Err(DeviceTreeError::UnterminatedString)
        };
        str::from_utf8(&slice[0..str_len]).map_err( DeviceTreeError::InvaildUTF8)
    }

    fn seek_u32(&self) -> Result<u32, DeviceTreeError> {
        if self.position + 4 > self.structure.len() {
            return Err(DeviceTreeError::OutOfStructureBounds);
        }
        let bytes = &self.structure[self.position..self.position + 4];
        bytes.try_into().map(u32::from_be_bytes).map_err(|_| DeviceTreeError::BytesParseError)
    }

    fn next_u32(&mut self) -> Result<u32, DeviceTreeError> {
        let result = self.seek_u32()?;
        self.position += 4;
        Ok(result)
    }

    fn proceed(&mut self, by: usize) -> Result<usize, DeviceTreeError> {
        self.position += by;
        if self.position % 4 != 0 {
            self.position = (self.position / 4 + 1) * 4;
        };
        if self.position >= self.structure.len() {
            return Err(DeviceTreeError::OutOfStructureBounds);
        }
        Ok(self.position)
    }

    fn skip_nop(&mut self) -> Result<(), DeviceTreeError>{
        while Self::NOP_NODE_TOKEN == self.seek_u32()? {
            self.next_u32()?;
        }
        Ok(())
    }
}
