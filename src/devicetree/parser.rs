use core::str;

#[derive(Debug)]
pub struct DeviceTreeParser<'a> {
    pub(super) structure: &'a [u8],
    pub(super) strings: &'a [u8],
    pub(super) position: usize
}

impl<'a> DeviceTreeParser<'a> {
    const START_NODE_TOKEN: u32 = 0x1;
    const END_NODE_TOKEN: u32 = 0x2;
    const PARAM_TOKEN: u32 = 0x3;
    const NOP_NODE_TOKEN: u32 = 0x4;
    const END_TOKEN: u32 = 0x9;

    pub fn next_param(&mut self) -> Option<(&str, &[u8])> {
        self.skip_nop();
        if self.seek_u32() == Some(Self::PARAM_TOKEN) {
            let _ = self.next_u32(); // move by param token
            let len = self.next_u32().expect("Cannot obtain param length") as usize;
            let nameoff = self.next_u32().expect("Cannot obtain param name offset") as usize;
            
            let name = Self::next_str(&self.strings[nameoff..]);
            let value = &self.structure[self.position..(self.position + len)];

            self.proceed(len);
            Some((name, value))
        } else {
            None
        }
    }

    pub fn next_node(&mut self) -> Option<&str> {
        self.skip_nop();
        if self.seek_u32() == Some(Self::START_NODE_TOKEN) {
            let _ = self.next_u32();
            self.next_str_local()
        } else {
            None
        }
    }

    pub fn next_node_end(&mut self) -> Option<()> {
        self.skip_nop();
        if self.seek_u32() == Some(Self::END_NODE_TOKEN) {
            let _ = self.next_u32();
            Some(())
        } else {
            None
        }

    }

    pub fn is_eof(&mut self) -> bool {
        self.skip_nop();
        self.seek_u32() == Some(Self::END_TOKEN)
    }

    fn next_str_local(&mut self) -> Option<&str> {
        self.skip_nop();
        let s = Self::next_str(&self.structure[self.position..]);
        self.proceed(s.bytes().len() + 1);
        Some(s)
    }

    fn next_str(slice: &[u8]) -> &str {
        let str_len = slice.iter().position(|&c| c == 0).unwrap();
        str::from_utf8(&slice[0..str_len]).unwrap_or("ERROR")
    }

    fn seek_u32(&self) -> Option<u32> {
        if self.position + 4 > self.structure.len() {
            return None;
        }
        let bytes = &self.structure[self.position..self.position + 4];
        bytes.try_into().map(u32::from_be_bytes).ok()
    }

    fn next_u32(&mut self) -> Option<u32> {
        let result = self.seek_u32();
        if result.is_some() {
            self.position += 4
        }
        result
    }

    fn proceed(&mut self, by: usize) {
        self.position += by;
        if self.position % 4 != 0 {
            self.position = (self.position / 4 + 1) * 4;
        }
    }

    fn skip_nop(&mut self) {
        while Some(Self::NOP_NODE_TOKEN) == self.seek_u32() {
            self.next_u32();
        }
    }
}