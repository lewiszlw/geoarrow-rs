use crate::array::{PointArray, WKBArray};
use crate::error::Result;
use crate::geo_traits::PointTrait;
use crate::io::wkb::reader::geometry::Endianness;
use crate::trait_::GeometryArrayTrait;
use arrow2::array::BinaryArray;
use arrow2::datatypes::DataType;
use arrow2::offset::Offsets;
use arrow2::types::Offset;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Write};

/// The byte length of a WKBPoint
pub const POINT_WKB_SIZE: usize = 1 + 4 + 8 + 8;

/// Write a Point geometry to a Writer encoded as WKB
pub fn write_point_as_wkb<W: Write>(mut writer: W, geom: impl PointTrait<T = f64>) -> Result<()> {
    // Byte order
    writer.write_u8(Endianness::LittleEndian.into()).unwrap();

    // wkbType = 1
    writer.write_u32::<LittleEndian>(1).unwrap();

    writer.write_f64::<LittleEndian>(geom.x()).unwrap();
    writer.write_f64::<LittleEndian>(geom.y()).unwrap();

    Ok(())
}

impl<O: Offset> From<&PointArray> for WKBArray<O> {
    fn from(value: &PointArray) -> Self {
        let non_null_count = value
            .validity()
            .map_or(value.len(), |validity| value.len() - validity.unset_bits());

        let validity = value.validity().cloned();
        // only allocate space for a WKBPoint for non-null items
        let values_len = non_null_count * POINT_WKB_SIZE;
        let mut offsets: Offsets<O> = Offsets::with_capacity(value.len());

        let values = {
            let values = Vec::with_capacity(values_len);
            let mut writer = Cursor::new(values);

            for maybe_geom in value.iter() {
                if let Some(geom) = maybe_geom {
                    write_point_as_wkb(&mut writer, geom).unwrap();
                    offsets.try_push_usize(POINT_WKB_SIZE).unwrap();
                } else {
                    offsets.extend_constant(1);
                }
            }

            writer.into_inner()
        };

        let data_type = match O::IS_LARGE {
            true => DataType::LargeBinary,
            false => DataType::Binary,
        };

        let binary_arr = BinaryArray::new(data_type, offsets.into(), values.into(), validity);
        WKBArray::new(binary_arr)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::point::{p0, p1, p2};

    #[test]
    fn round_trip() {
        // TODO: test with nulls
        let orig_arr: PointArray = vec![Some(p0()), Some(p1()), Some(p2())].into();
        let wkb_arr: WKBArray<i32> = (&orig_arr).into();
        let new_arr: PointArray = wkb_arr.try_into().unwrap();

        assert_eq!(orig_arr, new_arr);
    }

    #[test]
    fn round_trip_with_null() {
        let orig_arr: PointArray = vec![Some(p0()), None, Some(p1()), None, Some(p2())].into();
        let wkb_arr: WKBArray<i32> = (&orig_arr).into();
        let new_arr: PointArray = wkb_arr.try_into().unwrap();

        assert_eq!(orig_arr, new_arr);
    }
}