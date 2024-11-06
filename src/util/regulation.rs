use std::{collections::HashMap, io::Error, str::FromStr, sync::{Mutex, RwLock}};
use std::io::ErrorKind;
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use binary_reader::{BinaryReader, Endian};
use once_cell::sync::{Lazy, OnceCell};
use std::io::{self, Write};
use zune_inflate::{DeflateDecoder};

use crate::{db::{accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME, armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME, weapon_name::weapon_name::WEAPON_NAME}, save::save::save::Save, util::{param_structs::{EQUIP_PARAM_ACCESSORY_ST, EQUIP_PARAM_GEM_ST, EQUIP_PARAM_GOODS_ST, EQUIP_PARAM_PROTECTOR_ST, EQUIP_PARAM_WEAPON_ST}, params::params::{Row, PARAM}}};

use super::{bnd4::bnd4::BND4, params::params::Param};

pub static PARAMS: Lazy<RwLock<HashMap<Param, Vec<u8>>>> = Lazy::new(|| RwLock::new(Default::default()));

pub struct Regulation;

impl Regulation {
    pub fn init_params(save: &Save) {
        let res = Regulation::params_from_regulation(save.save_type.get_regulation());


        match res {
            Ok(res) => *PARAMS.write().unwrap() = res,
            Err(err) => println!("{err}"),
        }
    }

    pub fn equip_accessory_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_ACCESSORY_ST>> {
        static ACCESSORY_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_ACCESSORY_ST>>> = OnceCell::new();
        ACCESSORY_PARAM_MAP.get_or_init(|| {
            let mut map = Self::get_param_map::<EQUIP_PARAM_ACCESSORY_ST>(&Param::EquipParamAccessory);
            Self::try_fill_names::<EQUIP_PARAM_ACCESSORY_ST>(&mut map, &ACCESSORY_NAME);
            map
        })
    }

    pub fn equip_gem_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_GEM_ST>> {
        static GEM_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_GEM_ST>>> = OnceCell::new();
        GEM_PARAM_MAP.get_or_init(|| {
            let mut map = Self::get_param_map::<EQUIP_PARAM_GEM_ST>(&Param::EquipParamGem);
            Self::try_fill_names::<EQUIP_PARAM_GEM_ST>(&mut map, &AOW_NAME);
            map
        })
    }

    pub fn equip_goods_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_GOODS_ST>> {
        static GOOD_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_GOODS_ST>>> = OnceCell::new();
        GOOD_PARAM_MAP.get_or_init(|| {
            let mut map = Self::get_param_map::<EQUIP_PARAM_GOODS_ST>(&Param::EquipParamGoods);
            Self::try_fill_names::<EQUIP_PARAM_GOODS_ST>(&mut map, &ITEM_NAME);
            map
        })
    }

    pub fn equip_protectors_param_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_PROTECTOR_ST>> {
        static PROTECTOR_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_PROTECTOR_ST>>> = OnceCell::new();
        PROTECTOR_PARAM_MAP.get_or_init(|| {
            let mut map = Self::get_param_map::<EQUIP_PARAM_PROTECTOR_ST>(&Param::EquipParamProtector);
            Self::try_fill_names::<EQUIP_PARAM_PROTECTOR_ST>(&mut map, &ARMOR_NAME);
            map
        })
    }

    pub fn equip_weapon_params_map() -> &'static HashMap<u32, Row<EQUIP_PARAM_WEAPON_ST>> {
        static WEAPON_PARAM_MAP: OnceCell<HashMap<u32, Row<EQUIP_PARAM_WEAPON_ST>>> = OnceCell::new();
        WEAPON_PARAM_MAP.get_or_init(|| {
            let mut map = Self::get_param_map::<EQUIP_PARAM_WEAPON_ST>(&Param::EquipParamWeapon);
            Self::try_fill_names::<EQUIP_PARAM_WEAPON_ST>(&mut map, &WEAPON_NAME);
            map
        })
    }

    fn get_param_map<T>(param: &Param) -> HashMap<u32, Row<T>>
    where
        T: Default + Clone,
    {
        PARAM::<T>::from_bytes(&PARAMS.read().unwrap()[param])
            .unwrap()
            .rows.into_iter()
            .map(|row| (row.id, row))
            .collect::<HashMap<u32, Row<T>>>()
    }

    fn try_fill_names<T>(rows: &mut HashMap<u32, Row<T>>, map: &Lazy<Mutex<HashMap<u32, &str>>>)
    where
        T: Default + Clone,
    {
        rows.iter_mut().for_each(|(_, entry)| {
            entry.name = match map.lock().unwrap().get(&(entry.id)) {
                Some(name) => if !name.is_empty() { name.to_string() } else { format!("[UNKOWN_{}]", entry.id) },
                None => format!("[UNKOWN_{}]", entry.id),
            };
        });
    }

    pub fn check_save_compression(bytes: &[u8]) -> Result<bool, Error> {
        let decrypted = Self::decrypt(&bytes)?;
        Ok(Self::check_compression(&decrypted)?)
    }

    pub fn params_from_regulation(bytes: &[u8]) -> Result<HashMap<Param, Vec<u8>>, Error> {
        let decrypted = Self::decrypt(&bytes)?;
        let decompressed = Self::decompress(&decrypted)?;
        let res = Self::unpack(&decompressed)?;
        let mut params: HashMap<Param, Vec<u8>> = HashMap::new();

        for file in &res.files {
            let file_name = &file.name.split("\\").last().expect("Could not locate file name").split(".").nth(0).expect("Could not locate file name without extension");
            let param_type = Param::from_str(file_name)?;
            params.insert(param_type, file.bytes.to_vec());
        }
        Ok(params)
    }

    // Decrypt the regulation file (AES)
    fn decrypt(cipher_text: &[u8]) -> Result<Vec<u8>, Error> {
        type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
        let key = [0x99, 0xBF, 0xFC, 0x36, 0x6A, 0x6B, 0xC8, 0xC6, 0xF5, 0x82, 0x7D, 0x09, 0x36, 0x02, 0xD6, 0x76, 0xC4, 0x28, 0x92, 0xA0, 0x1C, 0x20, 0x7F, 0xB0, 0x24, 0xD3, 0xAF, 0x4E, 0x49, 0x3F, 0xEF, 0x99];
        let iv = &cipher_text[0..16];
        let mut buf = cipher_text[16..cipher_text.len()].to_vec();
        Aes256CbcDec::new(&key.into(), iv.into())
            .decrypt_padded_mut::<NoPadding>(&mut buf)
            .map_err(|e| Error::new(ErrorKind::Other, "upps"))
            .map(|pt| pt.to_vec())
    }
    
    fn check_bnd4_signature(data: &[u8]) -> bool {
        data.starts_with(b"BND4")
    }

    fn dump_decompressed_data(data: &[u8], filename: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(filename)?;
        file.write_all(data)?;
        eprintln!("Dumped decompressed data to {}", filename);
        Ok(())
    }

    fn decompress(bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let mut br = BinaryReader::from_u8(bytes);
        br.endian = Endian::Big;

        // Existing checks...
        assert_eq!(br.read_bytes(4)?, b"DCX\0");
        assert_eq!(br.read_i32()?, 0x11000);
        assert_eq!(br.read_i32()?, 0x18);
        assert_eq!(br.read_i32()?, 0x24);
        assert_eq!(br.read_i32()?, 0x44);
        assert_eq!(br.read_i32()?, 0x4c);

        assert_eq!(br.read_bytes(4)?, b"DCS\0");
        let decompressed_size = br.read_i32()?;
        let compressed_size = br.read_i32()?;

        assert_eq!(br.read_bytes(4)?, b"DCP\0");

        // Check for both ZSTD and DFLT
        let compression_type = br.read_bytes(4)?;
        let is_zstd = compression_type == b"ZSTD";
        let is_dflt = compression_type == b"DFLT";
        if !is_zstd && !is_dflt {
            return Err(Error::new(io::ErrorKind::InvalidData, "Unknown compression type"));
        }

        assert_eq!(br.read_i32()?, 0x20);

        // Read the compression level instead of asserting a specific value
        let compression_level = br.read_u8()?; // 0x15 for ZSTD, 0x09 for DFLT

        // Read the remaining header values without strict assertions
        let _unknown1 = br.read_u8()?;
        let _unknown2 = br.read_u8()?;
        let _unknown3 = br.read_u8()?;
        let _unknown4 = br.read_i32()?;
        let _unknown5 = br.read_u8()?;
        let _unknown6 = br.read_u8()?;
        let _unknown7 = br.read_u8()?;
        let _unknown8 = br.read_u8()?;
        let _unknown9 = br.read_i32()?;
        let _unknown10 = br.read_i32()?;

        assert_eq!(br.read_bytes(4)?, b"DCA\0");
        assert_eq!(br.read_i32()?, 8);

        let compressed = br.read_bytes(compressed_size as usize)?;
        #[cfg(debug_assertions)]
        {
            eprintln!("Compression type: {}", if is_zstd { "ZSTD" } else { "DFLT" });
            eprintln!("Compressed size: {}", compressed_size);
            eprintln!("Expected decompressed size: {}", decompressed_size);
            eprintln!("First few bytes of compressed data: {:?}", &compressed[..std::cmp::min(16, compressed.len())]);
        }
        // Decompress based on the compression type
        let decompressed = if is_zstd {
            match zstd::bulk::decompress(compressed, decompressed_size as usize) {
                Ok(data) => data,
                Err(e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("ZSTD decompression error: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::Other, format!("ZSTD decompression failed: {:?}", e)));
                }
            }
        } else {
            let mut decoder = DeflateDecoder::new(compressed);
            match decoder.decode_zlib() {
                Ok(data) => {
                    if Self::check_bnd4_signature(&data) {
                        return Ok(data);
                    } else {
                        #[cfg(debug_assertions)]
                        eprintln!("Decompressed data does not start with BND4 signature");
                        Self::dump_decompressed_data(&data, "failed_decompression.bin")?;
                    }
                }
                Err(e) => eprintln!("DEFLATE decompression error: {:?}", e),
            }

            return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to decompress DEFLATE data with all attempted methods"));
        };

        if !Self::check_bnd4_signature(&decompressed) {
            let error_msg = "Decompressed data does not start with BND4 signature";
            #[cfg(debug_assertions)]
            eprintln!("{}", error_msg);
            Self::dump_decompressed_data(&decompressed, "failed_decompression.bin")?;
            return Err(Error::new(io::ErrorKind::InvalidData, error_msg));
        }

        if decompressed.len() != decompressed_size as usize {
            let error_msg = format!("Decompressed size mismatch: expected {}, got {}", decompressed_size, decompressed.len());
            #[cfg(debug_assertions)]
            eprintln!("{}", error_msg);
            return Err(io::Error::new(io::ErrorKind::InvalidData, error_msg));
        }

        Ok(decompressed)
    }

    fn check_compression(bytes: &[u8]) -> Result<bool, Error> {
        let mut br = BinaryReader::from_u8(bytes);
        br.endian = Endian::Big;

        // Define a helper macro to reduce redundancy
        macro_rules! check {
            ($expr:expr, $expected:expr) => {
                if $expr? != $expected {
                    return Ok(false);
                }
            };
        }

        // Perform the existing checks using the helper macro
        check!(br.read_bytes(4), b"DCX\0");
        check!(br.read_i32(), 0x11000);
        check!(br.read_i32(), 0x18);
        check!(br.read_i32(), 0x24);
        check!(br.read_i32(), 0x44);
        check!(br.read_i32(), 0x4c);
        check!(br.read_bytes(4), b"DCS\0");

        // Read decompressed and compressed sizes (used later if needed)
        let decompressed_size = br.read_i32()?;
        let compressed_size = br.read_i32()?;

        // Check for compression type (either ZSTD or DFLT)
        check!(br.read_bytes(4), b"DCP\0");
        let compression_type = br.read_bytes(4)?;
        if compression_type != b"ZSTD" && compression_type != b"DFLT" {
            return Ok(false);
        }

        check!(br.read_i32(), 0x20);

        // Read the compression level (no specific assertion here)
        let _compression_level = br.read_u8()?;

        // Read remaining header values without strict assertions
        let _unknown1 = br.read_u8()?;
        let _unknown2 = br.read_u8()?;
        let _unknown3 = br.read_u8()?;
        let _unknown4 = br.read_i32()?;
        let _unknown5 = br.read_u8()?;
        let _unknown6 = br.read_u8()?;
        let _unknown7 = br.read_u8()?;
        let _unknown8 = br.read_u8()?;
        let _unknown9 = br.read_i32()?;
        let _unknown10 = br.read_i32()?;

        // Final checks
        check!(br.read_bytes(4), b"DCA\0");
        check!(br.read_i32(), 8);

        Ok(true)
    }

    // Unpack the decrypted and decompressed regulation file (BND4)
    fn unpack(bytes: &[u8]) -> Result<BND4, Error> {
        BND4::from_bytes(bytes)
    }
}