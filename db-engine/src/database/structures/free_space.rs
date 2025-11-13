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
        if begin >= end {
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

    pub fn deserialize_multiple(data: &[u8]) -> Result<Vec<Self>, &str> {
        if data.len() % FREE_SPACE_SECTION_SIZE != 0 {
            return Err("Data length must be a multiple of entry size");
        }
        let mut result = Vec::with_capacity(data.len() / FREE_SPACE_SECTION_SIZE);
        let mut remaining = data;

        while let Some((chunk, rest)) = remaining.split_first_chunk::<FREE_SPACE_SECTION_SIZE>() {
            result.push(Self::deserialize(chunk));
            remaining = rest;
        }

        if !remaining.is_empty() {
            return Err("Data is corrupted.");
        }

        Ok(result)
    }

    pub fn serialize(&self) -> [u8; FREE_SPACE_SECTION_SIZE] {
        let mut result: [u8; FREE_SPACE_SECTION_SIZE] = [0; FREE_SPACE_SECTION_SIZE];
        result[0..2].copy_from_slice(&self.page.to_le_bytes());
        result[2..4].copy_from_slice(&self.begin.to_le_bytes());
        result[4..6].copy_from_slice(&self.end.to_le_bytes());
        result
    }

    pub fn serialize_multiple(data: &[Self]) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::with_capacity(data.len() * FREE_SPACE_SECTION_SIZE);
        for free_space in data {
            result.extend_from_slice(&free_space.serialize());
        }
        result
    }

    pub fn begin(&self) -> u16 {
        self.begin
    }

    pub fn end(&self) -> u16 {
        self.end
    }

    pub fn free_space(&self) -> u16 {
        self.end - self.begin
    }

    pub fn move_begining(&mut self, shift: u16) -> Result<bool, ()> {
        if self.free_space() < shift {
            return Err(());
        }
        self.begin += shift;
        Ok(self.free_space() == 0)
    }

    pub fn extend_end(&mut self, shift: u16) {
        self.end += shift;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_usage() {
        let free_space1 = FreeSpace::new(1, 10, 11);
        let free_space2 = FreeSpace::new(0, 0, 10);
        assert_eq!(
            free_space1,
            Ok(FreeSpace {
                page: 1,
                begin: 10,
                end: 11
            })
        );
        assert_eq!(
            free_space2,
            Ok(FreeSpace {
                page: 0,
                begin: 0,
                end: 10
            })
        );
        assert_eq!(free_space1.unwrap().free_space(), 1);
        assert_eq!(free_space2.unwrap().free_space(), 10);
    }

    #[test]
    fn move_begining() {
        let mut free_space1 = FreeSpace::new(1, 10, 11).unwrap();
        assert_eq!(free_space1.move_begining(0), Ok(false));
        assert_eq!(free_space1.move_begining(2), Err(()));
        assert_eq!(free_space1.move_begining(1), Ok(true));
        assert_eq!(free_space1.move_begining(1), Err(()));
        assert_eq!(free_space1.move_begining(0), Ok(true));
        let mut free_space2 = FreeSpace::new(0, 0, 10).unwrap();
        assert_eq!(free_space2.move_begining(7), Ok(false));
        assert_eq!(free_space2.move_begining(4), Err(()));
        assert_eq!(free_space2.move_begining(2), Ok(false));
        assert_eq!(free_space2.move_begining(1), Ok(true));
    }

    #[test]
    fn wrong_format() {
        assert!(FreeSpace::new(1, 10, 10).is_err());
        assert!(FreeSpace::new(123, 4, 0).is_err());
    }

    // #[test]
    // fn serialization() {
    //     unimplemented!()
    // }

    #[test]
    fn serialization_deserialization() {
        let free_space_vec = vec![
            FreeSpace::new(0, 2, 10).unwrap(),
            FreeSpace::new(1, 0, 3).unwrap(),
            FreeSpace::new(3, 0, 10).unwrap(),
        ];
        for free_space in &free_space_vec {
            assert_eq!(FreeSpace::deserialize(&free_space.serialize()), *free_space);
        }
        assert_eq!(
            FreeSpace::deserialize_multiple(&FreeSpace::serialize_multiple(&free_space_vec))
                .unwrap(),
            free_space_vec
        )
    }
}
