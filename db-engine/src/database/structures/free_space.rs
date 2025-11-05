const U16_SIZE: usize = 2;
const FREE_SPACE_SECTION_SIZE: usize = 3 * U16_SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpace {
    pub page: u16,
    begin: u16,
    end: u16,
}

impl FreeSpace {
    pub fn new(page: u16, begin: u16, end: u16) -> Result<Self, &'static str> {
        if begin > end {
            Err("The begin index is bigger than the end index.")
        } else {
            Ok(FreeSpace { page, begin, end })
        }
    }

    pub fn deserialize(data: &[u8; FREE_SPACE_SECTION_SIZE]) -> Self {
        FreeSpace {
            page: u16::from_le_bytes([data[0], data[1]]),
            begin: u16::from_le_bytes([data[2], data[3]]),
            end: u16::from_le_bytes([data[4], data[5]]),
        }
    }

    pub fn serialize(&self) -> [u8; FREE_SPACE_SECTION_SIZE] {
        let mut result: [u8; FREE_SPACE_SECTION_SIZE] = [0; FREE_SPACE_SECTION_SIZE];
        result[0..2].copy_from_slice(&self.page.to_le_bytes());
        result[2..4].copy_from_slice(&self.begin.to_le_bytes());
        result[4..6].copy_from_slice(&self.end.to_le_bytes());
        result
    }

    pub fn begin(&self) -> u16 {
        self.begin
    }

    pub fn end(&self) -> u16 {
        self.end
    }
}
