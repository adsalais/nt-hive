// Copyright 2020-2025 Colin Finck <colin@reactos.org>
// SPDX-License-Identifier: GPL-2.0-or-later

use core::mem;
use core::ops::Range;
use core::ptr;

use bitflags::bitflags;
use enumn::N;
use memoffset::offset_of;
use zerocopy::byteorder::LittleEndian;
use zerocopy::{
    FromBytes, Immutable, IntoBytes, KnownLayout, Ref, SplitByteSlice, Unaligned, U16, U32,
};

use crate::big_data::{BigDataSlices, BIG_DATA_SEGMENT_SIZE};
use crate::error::{NtHiveError, Result};
use crate::helpers::byte_subrange;
use crate::hive::Hive;
use crate::string::NtHiveNameString;

#[cfg(feature = "alloc")]
use {
    alloc::{string::String, vec::Vec},
    core::{
        char::{self, DecodeUtf16, DecodeUtf16Error},
        iter::{self, FusedIterator, Map},
        slice::ChunksExact,
    },
};

/// This bit in `data_size` indicates that the data is small enough to be stored in `data_offset`.
const DATA_STORED_IN_DATA_OFFSET: u32 = 0x8000_0000;

bitflags! {
    struct KeyValueFlags: u16 {
        /// The name is in (extended) ASCII instead of UTF-16LE.
        const VALUE_COMP_NAME = 0x0001;
    }
}

/// Zero-copy representation of raw Key Value data, returned by [`KeyValue::data`].
#[derive(Clone)]
pub enum KeyValueData<'h, B: SplitByteSlice> {
    /// The data fits into a single cell.
    /// Contains the contiguous range of data bytes.
    Small(&'h [u8]),
    /// The data is big enough to require more than one cell.
    /// Contains an iterator that returns the data byte slice for each cell.
    Big(BigDataSlices<'h, B>),
}

impl<B> KeyValueData<'_, B>
where
    B: SplitByteSlice,
{
    #[cfg(feature = "alloc")]
    pub fn into_vec(self) -> Result<Vec<u8>> {
        match self {
            KeyValueData::Small(data) => Ok(data.to_vec()),
            KeyValueData::Big(iter) => {
                let mut data = Vec::new();

                for slice_data in iter {
                    let slice_data = slice_data?;
                    data.extend_from_slice(slice_data);
                }

                Ok(data)
            }
        }
    }
}

/// Possible data types of the data belonging to a [`KeyValue`].
#[derive(Clone, Copy, Debug, Eq, N, PartialEq)]
#[repr(u32)]
pub enum KeyValueDataType {
    RegNone = 0x0000_0000,
    RegSZ = 0x0000_0001,
    RegExpandSZ = 0x0000_0002,
    RegBinary = 0x0000_0003,
    RegDWord = 0x0000_0004,
    RegDWordBigEndian = 0x0000_0005,
    RegLink = 0x0000_0006,
    RegMultiSZ = 0x0000_0007,
    RegResourceList = 0x0000_0008,
    RegFullResourceDescriptor = 0x0000_0009,
    RegResourceRequirementsList = 0x0000_000a,
    RegQWord = 0x0000_000b,
}

/// On-Disk Structure of a Key Value header.
#[allow(dead_code)]
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned)]
#[repr(packed)]
struct KeyValueHeader {
    signature: [u8; 2],
    name_length: U16<LittleEndian>,
    data_size: U32<LittleEndian>,
    data_offset: U32<LittleEndian>,
    data_type: U32<LittleEndian>,
    flags: U16<LittleEndian>,
    spare: U16<LittleEndian>,
}

/// A single value that belongs to a [`KeyNode`].
/// It has a name and attached data.
///
/// On-Disk Signature: `vk`
///
/// [`KeyNode`]: crate::key_node::KeyNode
#[derive(Clone)]
pub struct KeyValue<'h, B: SplitByteSlice> {
    hive: &'h Hive<B>,
    header_range: Range<usize>,
    data_range: Range<usize>,
}

impl<'h, B> KeyValue<'h, B>
where
    B: SplitByteSlice,
{
    pub(crate) fn new(hive: &'h Hive<B>, cell_range: Range<usize>) -> Result<Self> {
        let header_range = byte_subrange(&cell_range, mem::size_of::<KeyValueHeader>())
            .ok_or_else(|| NtHiveError::InvalidHeaderSize {
                offset: hive.offset_of_data_offset(cell_range.start),
                expected: mem::size_of::<KeyValueHeader>(),
                actual: cell_range.len(),
            })?;
        let data_range = header_range.end..cell_range.end;

        let key_value = Self {
            hive,
            header_range,
            data_range,
        };
        key_value.validate_signature()?;

        Ok(key_value)
    }

    fn header(&self) -> Ref<&[u8], KeyValueHeader> {
        Ref::from_bytes(&self.hive.data[self.header_range.clone()]).unwrap()
    }

    /// Returns the raw data bytes as [`KeyValueData`].
    pub fn data(&self) -> Result<KeyValueData<'h, B>> {
        let header = self.header();

        let data_size = header.data_size.get();
        let data_stored_in_data_offset = data_size & DATA_STORED_IN_DATA_OFFSET > 0;
        let data_size = (data_size & !DATA_STORED_IN_DATA_OFFSET) as usize;

        if data_stored_in_data_offset {
            // If the entire data is stored in the `data_offset` field, its size mustn't
            // exceed the 4 bytes we have.
            if data_size > mem::size_of::<u32>() {
                return Err(NtHiveError::InvalidSizeField {
                    offset: self.hive.offset_of_field(&header.data_size),
                    expected: mem::size_of::<u32>(),
                    actual: data_size,
                });
            }

            let data_start = self.header_range.start + offset_of!(KeyValueHeader, data_offset);
            let data_end = data_start + data_size;

            Ok(KeyValueData::Small(&self.hive.data[data_start..data_end]))
        } else if data_size <= BIG_DATA_SEGMENT_SIZE {
            // The entire data is stored in a single cell referenced by `data_offset`.
            let cell_range = self
                .hive
                .cell_range_from_data_offset(header.data_offset.get())?;
            if cell_range.len() < data_size {
                return Err(NtHiveError::InvalidDataSize {
                    offset: self.hive.offset_of_data_offset(cell_range.start),
                    expected: data_size,
                    actual: cell_range.len(),
                });
            }

            let data_start = cell_range.start;
            let data_end = data_start + data_size;

            Ok(KeyValueData::Small(&self.hive.data[data_start..data_end]))
        } else {
            // The data size exceeds what can be stored in a single cell.
            // It's therefore stored in a Big Data structure referencing multiple cells.
            let cell_range = self
                .hive
                .cell_range_from_data_offset(header.data_offset.get())?;
            let iter = BigDataSlices::new(
                self.hive,
                data_size as u32,
                self.hive.offset_of_field(&header.data_size),
                cell_range,
            )?;

            Ok(KeyValueData::Big(iter))
        }
    }

    #[cfg(feature = "alloc")]
    fn utf16le_to_string_lossy<I>(iter: I) -> Result<String>
    where
        I: Iterator<Item = Result<&'h [u8]>>,
    {
        let mut string = String::new();

        // A very long REG_SZ / REG_EXPAND_SZ value may be split over several Big Data segments.
        // Transparently concatenate them in the output string.
        for slice_data in iter {
            let slice_data = slice_data?;

            let u16_iter = slice_data
                .chunks_exact(2)
                .map(|two_bytes| u16::from_le_bytes(two_bytes.try_into().unwrap()));

            // You hardly find emojis or other characters outside the UTF-16 Basic Multilingual Plane in registry data.
            // Hence, the count of UTF-16 code points is a good estimate for the final string length.
            string.reserve(u16_iter.len());

            // Interpret the u16 chunks as UTF-16 code points for characters. Replace undecodable ones silently.
            let char_iter =
                char::decode_utf16(u16_iter).map(|x| x.unwrap_or(char::REPLACEMENT_CHARACTER));

            for c in char_iter {
                // Some applications erroneously store NUL-terminated strings in the registry.
                // To cope with that, we either stop at the first NUL character or when no more characters are left, whatever comes first.
                if c == '\0' {
                    return Ok(string);
                } else {
                    string.push(c);
                }
            }
        }

        Ok(string)
    }

    /// Checks if this is a `REG_SZ` or `REG_EXPAND_SZ` Key Value
    /// and returns the data as a [`String`] in that case.
    #[cfg(feature = "alloc")]
    pub fn string_data(&'h self) -> Result<String> {
        match self.data_type()? {
            KeyValueDataType::RegSZ | KeyValueDataType::RegExpandSZ => (),
            data_type => {
                return Err(NtHiveError::InvalidKeyValueDataType {
                    expected: &[KeyValueDataType::RegSZ, KeyValueDataType::RegExpandSZ],
                    actual: data_type,
                });
            }
        }

        match self.data()? {
            KeyValueData::Small(data) => Self::utf16le_to_string_lossy(iter::once(Ok(data))),
            KeyValueData::Big(iter) => Self::utf16le_to_string_lossy(iter),
        }
    }

    /// Checks if this is a `REG_DWORD` or `REG_DWORD_BIG_ENDIAN` Key Value
    /// and returns the data as a [`u32`] in that case.
    pub fn dword_data(&self) -> Result<u32> {
        // DWORD data never needs a Big Data structure.
        if let KeyValueData::Small(data) = self.data()? {
            // DWORD data must be exactly 4 bytes long.
            if data.len() != mem::size_of::<u32>() {
                return Err(NtHiveError::InvalidDataSize {
                    offset: self.hive.offset_of_field(&data),
                    expected: mem::size_of::<u32>(),
                    actual: data.len(),
                });
            }

            // Ensure that this is a REG_DWORD or REG_DWORD_BIG_ENDIAN data type.
            match self.data_type()? {
                KeyValueDataType::RegDWord => Ok(u32::from_le_bytes(data.try_into().unwrap())),
                KeyValueDataType::RegDWordBigEndian => {
                    Ok(u32::from_be_bytes(data.try_into().unwrap()))
                }
                data_type => Err(NtHiveError::InvalidKeyValueDataType {
                    expected: &[
                        KeyValueDataType::RegDWord,
                        KeyValueDataType::RegDWordBigEndian,
                    ],
                    actual: data_type,
                }),
            }
        } else {
            // We got a Big Data structure and this can only happen if the data
            // is much longer than a single DWORD.
            Err(NtHiveError::InvalidDataSize {
                offset: self
                    .hive
                    .offset_of_data_offset(self.header().data_offset.get() as usize),
                expected: mem::size_of::<u32>(),
                actual: self.data_size() as usize,
            })
        }
    }

    /// Checks if this is a `REG_MULTI_SZ` Key Value
    /// and returns an iterator over [`String`]s for each line in that case.
    #[cfg(feature = "alloc")]
    pub fn multi_string_data(&self) -> Result<RegMultiSZStrings<'h, B>> {
        // Ensure that this is a REG_MULTI_SZ data type.
        match self.data_type()? {
            KeyValueDataType::RegMultiSZ => (),
            data_type => {
                return Err(NtHiveError::InvalidKeyValueDataType {
                    expected: &[KeyValueDataType::RegMultiSZ],
                    actual: data_type,
                });
            }
        }

        match self.data()? {
            KeyValueData::Small(data) => Ok(RegMultiSZStrings::small(data)),
            KeyValueData::Big(iter) => Ok(RegMultiSZStrings::big(iter)),
        }
    }

    /// Checks if this is a `REG_QWORD` Key Value
    /// and returns the data as a [`u64`] in that case.
    pub fn qword_data(&self) -> Result<u64> {
        // QWORD data never needs a Big Data structure.
        if let KeyValueData::Small(data) = self.data()? {
            // QWORD data must be exactly 8 bytes long.
            if data.len() != mem::size_of::<u64>() {
                return Err(NtHiveError::InvalidDataSize {
                    offset: self.hive.offset_of_field(&data),
                    expected: mem::size_of::<u64>(),
                    actual: data.len(),
                });
            }

            // Ensure that this is a REG_QWORD data type.
            match self.data_type()? {
                KeyValueDataType::RegQWord => Ok(u64::from_le_bytes(data.try_into().unwrap())),
                data_type => Err(NtHiveError::InvalidKeyValueDataType {
                    expected: &[KeyValueDataType::RegQWord],
                    actual: data_type,
                }),
            }
        } else {
            // We got a Big Data structure and this can only happen if the data
            // is much longer than a single QWORD.
            Err(NtHiveError::InvalidDataSize {
                offset: self
                    .hive
                    .offset_of_data_offset(self.header().data_offset.get() as usize),
                expected: mem::size_of::<u64>(),
                actual: self.data_size() as usize,
            })
        }
    }

    /// Returns the size of the raw data.
    pub fn data_size(&self) -> u32 {
        let header = self.header();
        header.data_size.get() & !DATA_STORED_IN_DATA_OFFSET
    }

    /// Returns the data type of this Key Value.
    pub fn data_type(&self) -> Result<KeyValueDataType> {
        let header = self.header();
        let data_type_code = header.data_type.get();

        KeyValueDataType::n(data_type_code).ok_or_else(|| {
            NtHiveError::UnsupportedKeyValueDataType {
                offset: self.hive.offset_of_field(&header.data_type),
                actual: data_type_code,
            }
        })
    }

    /// Returns the name of this Key Value.
    pub fn name(&self) -> Result<NtHiveNameString<'h>> {
        let header = self.header();
        let flags = KeyValueFlags::from_bits_truncate(header.flags.get());
        let name_length = header.name_length.get() as usize;

        let name_range = byte_subrange(&self.data_range, name_length).ok_or_else(|| {
            NtHiveError::InvalidSizeField {
                offset: self.hive.offset_of_field(&header.name_length),
                expected: name_length,
                actual: self.data_range.len(),
            }
        })?;
        let name_bytes = &self.hive.data[name_range];

        if flags.contains(KeyValueFlags::VALUE_COMP_NAME) {
            Ok(NtHiveNameString::Latin1(name_bytes))
        } else {
            Ok(NtHiveNameString::Utf16LE(name_bytes))
        }
    }

    fn validate_signature(&self) -> Result<()> {
        let header = self.header();
        let signature = &header.signature;
        let expected_signature = b"vk";

        if signature == expected_signature {
            Ok(())
        } else {
            Err(NtHiveError::InvalidTwoByteSignature {
                offset: self.hive.offset_of_field(signature),
                expected: expected_signature,
                actual: *signature,
            })
        }
    }
}

impl<B> PartialEq for KeyValue<'_, B>
where
    B: SplitByteSlice,
{
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.hive, other.hive)
            && self.header_range == other.header_range
            && self.data_range == other.data_range
    }
}

impl<B> Eq for KeyValue<'_, B> where B: SplitByteSlice {}

#[cfg(feature = "alloc")]
type RegMultiSZCharIter<'h> = Map<
    DecodeUtf16<Map<ChunksExact<'h, u8>, fn(&'h [u8]) -> u16>>,
    fn(Result<char, DecodeUtf16Error>) -> char,
>;

#[cfg(feature = "alloc")]
#[derive(Clone)]
pub struct RegMultiSZStrings<'h, B>
where
    B: SplitByteSlice + 'h,
{
    char_iter: Option<RegMultiSZCharIter<'h>>,
    big_iter: Option<BigDataSlices<'h, B>>,
}

#[cfg(feature = "alloc")]
impl<'h, B> RegMultiSZStrings<'h, B>
where
    B: SplitByteSlice + 'h,
{
    fn small(data: &'h [u8]) -> Self {
        Self {
            char_iter: Some(Self::make_char_iter(data)),
            big_iter: None,
        }
    }

    fn big(iter: BigDataSlices<'h, B>) -> Self {
        Self {
            char_iter: None,
            big_iter: Some(iter),
        }
    }

    fn make_char_iter(slice_data: &'h [u8]) -> RegMultiSZCharIter<'h> {
        let u16_iter = slice_data
            .chunks_exact(2)
            .map(Self::u16_from_le_bytes as fn(&[u8]) -> u16);
        char::decode_utf16(u16_iter).map(
            Self::unwrap_or_replacement_character as fn(Result<char, DecodeUtf16Error>) -> char,
        )
    }

    fn u16_from_le_bytes(two_bytes: &[u8]) -> u16 {
        u16::from_le_bytes(two_bytes.try_into().unwrap())
    }

    fn unwrap_or_replacement_character(input: Result<char, DecodeUtf16Error>) -> char {
        input.unwrap_or(char::REPLACEMENT_CHARACTER)
    }
}

#[cfg(feature = "alloc")]
impl<'h, B> Iterator for RegMultiSZStrings<'h, B>
where
    B: SplitByteSlice + 'h,
{
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut string = String::new();

        'outer_loop: loop {
            let char_iter = match self.char_iter.as_mut() {
                Some(char_iter) => char_iter,
                None => {
                    let big_iter = match self.big_iter.as_mut() {
                        Some(big_iter) => big_iter,
                        None => break 'outer_loop,
                    };
                    let slice_data = match big_iter.next() {
                        Some(Ok(slice_data)) => slice_data,
                        Some(Err(e)) => return Some(Err(e)),
                        None => break 'outer_loop,
                    };
                    let char_iter = Self::make_char_iter(slice_data);
                    self.char_iter = Some(char_iter);
                    continue 'outer_loop;
                }
            };

            for c in char_iter {
                // REG_MULTI_SZ data consists of multiple strings each terminated by a NUL character.
                // The final string has a double-NUL termination.
                //
                // However, we will happily accept data without terminating NUL characters as well.
                if c == '\0' {
                    break 'outer_loop;
                } else {
                    string.push(c);
                }
            }

            // We have fully iterated all characters of this slice.
            // Get a new `char_iter` in the next iteration of the outer loop, and concatenate characters
            // to our `string` until we find a NUL or no more data.
            self.char_iter = None;
        }

        if string.is_empty() {
            None
        } else {
            Some(Ok(string))
        }
    }
}

#[cfg(feature = "alloc")]
impl<'h, B> FusedIterator for RegMultiSZStrings<'h, B> where B: SplitByteSlice + 'h {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_data() {
        // Get Key Values of all data types we support and prove that we correctly
        // read their data.
        let testhive = crate::helpers::tests::testhive_vec();
        let hive = Hive::new(testhive.as_ref()).unwrap();
        let root_key_node = hive.root_key_node().unwrap();
        let key_node = root_key_node.subkey("data-test").unwrap().unwrap();

        let key_value = key_node.value("reg-sz").unwrap().unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegSZ);
        assert_eq!(key_value.string_data().unwrap(), "sz-test");

        let key_value = key_node
            .value("reg-sz-with-terminating-nul")
            .unwrap()
            .unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegSZ);
        assert_eq!(key_value.string_data().unwrap(), "sz-test");

        let key_value = key_node.value("reg-expand-sz").unwrap().unwrap();
        assert_eq!(
            key_value.data_type().unwrap(),
            KeyValueDataType::RegExpandSZ
        );
        assert_eq!(key_value.string_data().unwrap(), "sz-test");

        let key_value = key_node.value("reg-multi-sz").unwrap().unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegMultiSZ);
        let mut iter = key_value.multi_string_data().unwrap();
        assert_eq!(iter.next(), Some(Ok("multi-sz-test".to_owned())));
        assert_eq!(iter.next(), Some(Ok("line2".to_owned())));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let key_value = key_node.value("reg-multi-sz-big").unwrap().unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegMultiSZ);
        let mut iter = key_value.multi_string_data().unwrap();
        assert_eq!(iter.next(), Some(Ok("0123456789".repeat(820))));
        assert_eq!(iter.next(), Some(Ok("0123456789".to_owned())));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let key_value = key_node.value("dword").unwrap().unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegDWord);
        assert_eq!(key_value.dword_data().unwrap(), 42);

        // offreg-testhive-writer has stored the same bytes representing '42' in
        // little-endian for the big-endian case.
        // Thus, we must get a numeric value of 42 << 24 = 704643072 after
        // interpreting the same bytes as a big-endian value.
        let key_value = key_node.value("dword-big-endian").unwrap().unwrap();
        assert_eq!(
            key_value.data_type().unwrap(),
            KeyValueDataType::RegDWordBigEndian
        );
        assert_eq!(key_value.dword_data().unwrap(), 42 << 24);

        let key_value = key_node.value("qword").unwrap().unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegQWord);
        assert_eq!(key_value.qword_data().unwrap(), u64::MAX);

        let key_value = key_node.value("binary").unwrap().unwrap();
        let key_value_data = key_value.data().unwrap();
        assert_eq!(key_value.data_type().unwrap(), KeyValueDataType::RegBinary);
        assert!(matches!(key_value_data, KeyValueData::Small(_)));
        assert_eq!(key_value_data.into_vec().unwrap(), vec![1, 2, 3, 4, 5]);
    }
}
