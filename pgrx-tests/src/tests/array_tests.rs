/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgrx::array::RawArray;
use pgrx::prelude::*;
use pgrx::{Array, Json};
use serde_json::*;

#[pg_extern(name = "sum_array")]
fn sum_array_i32(values: Array<i32>) -> i32 {
    // we implement it this way so we can trap an overflow (as we have a test for this) and
    // catch it correctly in both --debug and --release modes
    let mut sum = 0_i32;
    for v in values {
        let v = v.unwrap_or(0);
        let (val, overflow) = sum.overflowing_add(v);
        if overflow {
            panic!("attempt to add with overflow");
        } else {
            sum = val;
        }
    }
    sum
}

#[pg_extern(name = "sum_array")]
fn sum_array_i64(values: Array<i64>) -> i64 {
    values.iter().map(|v| v.unwrap_or(0i64)).sum()
}

#[pg_extern]
fn count_true(values: Array<bool>) -> i32 {
    values.iter().filter(|b| b.unwrap_or(false)).count() as i32
}

#[pg_extern]
fn count_nulls(values: Array<i32>) -> i32 {
    values.iter().map(|v| v.is_none()).filter(|v| *v).count() as i32
}

#[pg_extern]
fn optional_array_arg(values: Option<Array<f32>>) -> f32 {
    values.unwrap().iter().map(|v| v.unwrap_or(0f32)).sum()
}

#[pg_extern]
fn iterate_array_with_deny_null(values: Array<i32>) {
    for _ in values.iter_deny_null() {
        // noop
    }
}

#[pg_extern]
fn optional_array_with_default(values: default!(Option<Array<i32>>, "NULL")) -> i32 {
    values.unwrap().iter().map(|v| v.unwrap_or(0)).sum()
}

#[pg_extern]
fn serde_serialize_array(values: Array<&str>) -> Json {
    Json(json! { { "values": values } })
}

#[pg_extern]
fn serde_serialize_array_i32(values: Array<i32>) -> Json {
    Json(json! { { "values": values } })
}

#[pg_extern]
fn serde_serialize_array_i32_deny_null(values: Array<i32>) -> Json {
    Json(json! { { "values": values.iter_deny_null() } })
}

#[pg_extern]
fn return_text_array() -> Vec<&'static str> {
    vec!["a", "b", "c", "d"]
}

#[pg_extern]
fn return_zero_length_vec() -> Vec<i32> {
    Vec::new()
}

#[pg_extern]
fn get_arr_nelems(arr: Array<i32>) -> libc::c_int {
    // SAFETY: Eh it's fine, it's just a len check.
    unsafe { RawArray::from_array(arr) }.unwrap().len() as _
}

#[pg_extern]
fn get_arr_data_ptr_nth_elem(arr: Array<i32>, elem: i32) -> Option<i32> {
    // SAFETY: this is Known to be an Array from ArrayType,
    // and it's valid-ish to see any bitpattern of an i32 inbounds of a slice.
    unsafe {
        let raw = RawArray::from_array(arr).unwrap().data::<i32>();
        let slice = &(*raw.as_ptr());
        slice.get(elem as usize).copied()
    }
}

#[pg_extern]
fn display_get_arr_nullbitmap(arr: Array<i32>) -> String {
    let mut raw = unsafe { RawArray::from_array(arr) }.unwrap();

    if let Some(slice) = raw.nulls() {
        // SAFETY: If the test has gotten this far, the ptr is good for 0+ bytes,
        // so reborrow NonNull<[u8]> as &[u8] for the hot second we're looking at it.
        let slice = unsafe { &*slice.as_ptr() };
        // might panic if the array is len 0
        format!("{:#010b}", slice[0])
    } else {
        String::from("")
    }
}

#[pg_extern]
fn get_arr_ndim(arr: Array<i32>) -> libc::c_int {
    // SAFETY: This is a valid ArrayType and it's just a field access.
    unsafe { RawArray::from_array(arr) }.unwrap().dims().len() as _
}

// This deliberately iterates the Array.
// Because Array::iter currently iterates the Array as Datums, this is guaranteed to be "bug-free" regarding size.
#[pg_extern]
fn arr_mapped_vec(arr: Array<i32>) -> Vec<i32> {
    arr.iter().filter_map(|x| x).collect()
}

/// Naive conversion.
#[pg_extern]
#[allow(deprecated)]
fn arr_into_vec(arr: Array<i32>) -> Vec<i32> {
    arr.iter_deny_null().collect()
}

#[pg_extern]
#[allow(deprecated)]
fn arr_sort_uniq(arr: Array<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = arr.iter_deny_null().collect();
    v.sort();
    v.dedup();
    v
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;
    use pgrx::{IntoDatum, Json};
    use serde_json::json;

    #[pg_test]
    fn test_sum_array_i32() {
        let sum = Spi::get_one::<i32>("SELECT sum_array(ARRAY[1,2,3]::integer[])");
        assert_eq!(sum, Ok(Some(6)));
    }

    #[pg_test]
    fn test_sum_array_i64() {
        let sum = Spi::get_one::<i64>("SELECT sum_array(ARRAY[1,2,3]::bigint[])");
        assert_eq!(sum, Ok(Some(6)));
    }

    #[pg_test(error = "attempt to add with overflow")]
    fn test_sum_array_i32_overflow() -> Result<Option<i64>, pgrx::spi::Error> {
        Spi::get_one::<i64>(
            "SELECT sum_array(a) FROM (SELECT array_agg(s) a FROM generate_series(1, 1000000) s) x;",
        )
    }

    #[pg_test]
    fn test_count_true() {
        let cnt = Spi::get_one::<i32>("SELECT count_true(ARRAY[true, true, false, true])");
        assert_eq!(cnt, Ok(Some(3)));
    }

    #[pg_test]
    fn test_count_nulls() {
        let cnt = Spi::get_one::<i32>("SELECT count_nulls(ARRAY[NULL, 1, 2, NULL]::integer[])");
        assert_eq!(cnt, Ok(Some(2)));
    }

    #[pg_test]
    fn test_optional_array() {
        let sum = Spi::get_one::<f32>("SELECT optional_array_arg(ARRAY[1,2,3]::real[])");
        assert_eq!(sum, Ok(Some(6f32)));
    }

    #[pg_test(error = "array contains NULL")]
    fn test_array_deny_nulls() -> Result<(), spi::Error> {
        Spi::run("SELECT iterate_array_with_deny_null(ARRAY[1,2,3, NULL]::int[])")
    }

    #[pg_test]
    fn test_serde_serialize_array() -> Result<(), pgrx::spi::Error> {
        let json = Spi::get_one::<Json>(
            "SELECT serde_serialize_array(ARRAY['one', null, 'two', 'three'])",
        )?
        .expect("returned json was null");
        assert_eq!(json.0, json! {{"values": ["one", null, "two", "three"]}});
        Ok(())
    }

    #[pg_test]
    fn test_optional_array_with_default() {
        let sum = Spi::get_one::<i32>("SELECT optional_array_with_default(ARRAY[1,2,3])");
        assert_eq!(sum, Ok(Some(6)));
    }

    #[pg_test]
    fn test_serde_serialize_array_i32() -> Result<(), pgrx::spi::Error> {
        let json = Spi::get_one::<Json>(
            "SELECT serde_serialize_array_i32(ARRAY[1, null, 2, 3, null, 4, 5])",
        )?
        .expect("returned json was null");
        assert_eq!(json.0, json! {{"values": [1,null,2,3,null,4, 5]}});
        Ok(())
    }

    #[pg_test(error = "array contains NULL")]
    fn test_serde_serialize_array_i32_deny_null() -> Result<Option<Json>, pgrx::spi::Error> {
        Spi::get_one::<Json>(
            "SELECT serde_serialize_array_i32_deny_null(ARRAY[1, 2, 3, null, 4, 5])",
        )
    }

    #[pg_test]
    fn test_return_text_array() {
        let rc = Spi::get_one::<bool>("SELECT ARRAY['a', 'b', 'c', 'd'] = return_text_array();");
        assert_eq!(rc, Ok(Some(true)));
    }

    #[pg_test]
    fn test_return_zero_length_vec() {
        let rc = Spi::get_one::<bool>("SELECT ARRAY[]::integer[] = return_zero_length_vec();");
        assert_eq!(rc, Ok(Some(true)));
    }

    #[pg_test]
    fn test_slice_to_array() -> Result<(), pgrx::spi::Error> {
        let owned_vec = vec![Some(1), None, Some(2), Some(3), None, Some(4), Some(5)];
        let json = Spi::connect(|client| {
            client
                .select(
                    "SELECT serde_serialize_array_i32($1)",
                    None,
                    Some(vec![(
                        PgBuiltInOids::INT4ARRAYOID.oid(),
                        owned_vec.as_slice().into_datum(),
                    )]),
                )?
                .first()
                .get_one::<Json>()
        })?
        .expect("Failed to return json even though it's right there ^^");
        assert_eq!(json.0, json! {{"values": [1, null, 2, 3, null, 4, 5]}});
        Ok(())
    }

    #[pg_test]
    fn test_arr_data_ptr() {
        let len = Spi::get_one::<i32>("SELECT get_arr_nelems('{1,2,3,4,5}'::int[])");
        assert_eq!(len, Ok(Some(5)));
    }

    #[pg_test]
    fn test_get_arr_data_ptr_nth_elem() {
        let nth = Spi::get_one::<i32>("SELECT get_arr_data_ptr_nth_elem('{1,2,3,4,5}'::int[], 2)");
        assert_eq!(nth, Ok(Some(3)));
    }

    #[pg_test]
    fn test_display_get_arr_nullbitmap() -> Result<(), pgrx::spi::Error> {
        let bitmap_str = Spi::get_one::<String>(
            "SELECT display_get_arr_nullbitmap(ARRAY[1,NULL,3,NULL,5]::int[])",
        )?
        .expect("datum was null");

        assert_eq!(bitmap_str, "0b00010101");

        let bitmap_str =
            Spi::get_one::<String>("SELECT display_get_arr_nullbitmap(ARRAY[1,2,3,4,5]::int[])")?
                .expect("datum was null");

        assert_eq!(bitmap_str, "");
        Ok(())
    }

    #[pg_test]
    fn test_get_arr_ndim() -> Result<(), pgrx::spi::Error> {
        let ndim = Spi::get_one::<i32>("SELECT get_arr_ndim(ARRAY[1,2,3,4,5]::int[])")?
            .expect("datum was null");

        assert_eq!(ndim, 1);

        let ndim = Spi::get_one::<i32>("SELECT get_arr_ndim('{{1,2,3},{4,5,6}}'::int[])")?
            .expect("datum was null");

        assert_eq!(ndim, 2);
        Ok(())
    }

    #[pg_test]
    fn test_arr_to_vec() {
        let result = Spi::get_one::<Vec<i32>>("SELECT arr_mapped_vec(ARRAY[3,2,2,1]::integer[])");
        let other = Spi::get_one::<Vec<i32>>("SELECT arr_into_vec(ARRAY[3,2,2,1]::integer[])");
        // One should be equivalent to the canonical form.
        assert_eq!(result, Ok(Some(vec![3, 2, 2, 1])));
        // And they should be equal to each other.
        assert_eq!(result, other);
    }

    #[pg_test]
    fn test_arr_sort_uniq() {
        let result = Spi::get_one::<Vec<i32>>("SELECT arr_sort_uniq(ARRAY[3,2,2,1]::integer[])");
        assert_eq!(result, Ok(Some(vec![1, 2, 3])));
    }

    #[pg_test]
    #[should_panic]
    fn test_arr_sort_uniq_with_null() -> Result<(), pgrx::spi::Error> {
        Spi::get_one::<Vec<i32>>("SELECT arr_sort_uniq(ARRAY[3,2,NULL,2,1]::integer[])").map(|_| ())
    }
}