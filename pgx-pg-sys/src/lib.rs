//
// we allow improper_ctypes just to eliminate these warnings:
//      = note: `#[warn(improper_ctypes)]` on by default
//      = note: 128-bit integers don't currently have a known stable ABI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(clippy::unneeded_field_pattern)]

pub mod submodules;
pub use submodules::guard;
pub use submodules::*;

//
// our actual bindings modules -- these are generated by build.rs
//

mod common;

// always make the version specific modules public, regardless of how we're feature gated
pub mod pg10_specific;
pub mod pg11_specific;
pub mod pg12_specific;

// expose things we want available for all versions
pub use all_versions::*;

// and things that are version-specific
pub use internal::*;

/// item declarations we want to add to all versions
mod all_versions {
    use crate as pg_sys;
    use pgx_macros::*;

    use memoffset::*;
    use std::str::FromStr;

    /// this comes from `postgres_ext.h`
    pub const InvalidOid: super::Oid = 0;
    pub const InvalidOffsetNumber: super::OffsetNumber = 0;
    pub const FirstOffsetNumber: super::OffsetNumber = 1;
    pub const MaxOffsetNumber: super::OffsetNumber =
        (super::BLCKSZ as usize / std::mem::size_of::<super::ItemIdData>()) as super::OffsetNumber;
    pub const InvalidBlockNumber: u32 = 0xFFFF_FFFF as crate::BlockNumber;
    pub const VARHDRSZ: usize = std::mem::size_of::<super::int32>();
    pub const InvalidTransactionId: super::TransactionId = 0 as super::TransactionId;
    pub const InvalidCommandId: super::CommandId = 0 as super::CommandId;
    pub const BootstrapTransactionId: super::TransactionId = 1 as super::TransactionId;
    pub const FrozenTransactionId: super::TransactionId = 2 as super::TransactionId;
    pub const FirstNormalTransactionId: super::TransactionId = 3 as super::TransactionId;
    pub const MaxTransactionId: super::TransactionId = 0xFFFF_FFFF as super::TransactionId;

    #[inline]
    pub fn VARHDRSZ_EXTERNAL() -> usize {
        offset_of!(super::varattrib_1b_e, va_data)
    }

    #[inline]
    pub fn VARHDRSZ_SHORT() -> usize {
        offset_of!(super::varattrib_1b, va_data)
    }

    #[inline]
    pub fn get_pg_major_version_string() -> &'static str {
        let mver = std::ffi::CStr::from_bytes_with_nul(super::PG_MAJORVERSION).unwrap();
        mver.to_str().unwrap()
    }

    #[inline]
    pub fn get_pg_major_version_num() -> u16 {
        u16::from_str(super::get_pg_major_version_string()).unwrap()
    }

    #[inline]
    pub fn get_pg_version_string() -> &'static str {
        let ver = std::ffi::CStr::from_bytes_with_nul(super::PG_VERSION_STR).unwrap();
        ver.to_str().unwrap()
    }

    #[inline]
    pub fn get_pg_major_minor_version_string() -> &'static str {
        let mver = std::ffi::CStr::from_bytes_with_nul(super::PG_VERSION).unwrap();
        mver.to_str().unwrap()
    }

    #[inline]
    pub fn TransactionIdIsNormal(xid: super::TransactionId) -> bool {
        xid >= FirstNormalTransactionId
    }

    /// ```c
    ///     #define type_is_array(typid)  (get_element_type(typid) != InvalidOid)
    /// ```
    #[inline]
    pub unsafe fn type_is_array(typoid: super::Oid) -> bool {
        super::get_element_type(typoid) != InvalidOid
    }

    #[inline]
    pub unsafe fn planner_rt_fetch(
        index: super::Index,
        root: *mut super::PlannerInfo,
    ) -> *mut super::RangeTblEntry {
        extern "C" {
            pub fn pgx_planner_rt_fetch(
                index: super::Index,
                root: *mut super::PlannerInfo,
            ) -> *mut super::RangeTblEntry;
        }

        pgx_planner_rt_fetch(index, root)
    }

    /// ```c
    /// #define rt_fetch(rangetable_index, rangetable) \
    ///     ((RangeTblEntry *) list_nth(rangetable, (rangetable_index)-1))
    /// ```
    #[inline]
    pub fn rt_fetch(
        index: super::Index,
        range_table: *mut super::List,
    ) -> *mut super::RangeTblEntry {
        unsafe { super::list_nth(range_table, index as i32 - 1) as *mut super::RangeTblEntry }
    }

    #[inline]
    pub fn HeapTupleHeaderGetXmin(
        htup_header: super::HeapTupleHeader,
    ) -> Option<super::TransactionId> {
        extern "C" {
            pub fn pgx_HeapTupleHeaderGetXmin(
                htup_header: super::HeapTupleHeader,
            ) -> super::TransactionId;
        }

        if htup_header.is_null() {
            None
        } else {
            Some(unsafe { pgx_HeapTupleHeaderGetXmin(htup_header) })
        }
    }

    #[inline]
    pub fn HeapTupleHeaderGetRawCommandId(
        htup_header: super::HeapTupleHeader,
    ) -> Option<super::CommandId> {
        extern "C" {
            pub fn pgx_HeapTupleHeaderGetRawCommandId(
                htup_header: super::HeapTupleHeader,
            ) -> super::CommandId;
        }

        if htup_header.is_null() {
            None
        } else {
            Some(unsafe { pgx_HeapTupleHeaderGetRawCommandId(htup_header) })
        }
    }

    #[inline]
    pub fn HeapTupleHeaderIsHeapOnly(htup_header: super::HeapTupleHeader) -> bool {
        extern "C" {
            pub fn pgx_HeapTupleHeaderIsHeapOnly(htup_header: super::HeapTupleHeader) -> bool;
        }

        if htup_header.is_null() {
            panic!("provided HeapTupleHeader is null");
        } else {
            unsafe { pgx_HeapTupleHeaderIsHeapOnly(htup_header) }
        }
    }

    #[pg_guard]
    extern "C" {
        pub fn query_tree_walker(
            query: *mut super::Query,
            walker: ::std::option::Option<
                unsafe extern "C" fn(*mut super::Node, *mut ::std::os::raw::c_void) -> bool,
            >,
            context: *mut ::std::os::raw::c_void,
            flags: ::std::os::raw::c_int,
        ) -> bool;
    }

    #[pg_guard]
    extern "C" {
        pub fn expression_tree_walker(
            node: *mut super::Node,
            walker: ::std::option::Option<
                unsafe extern "C" fn(*mut super::Node, *mut ::std::os::raw::c_void) -> bool,
            >,
            context: *mut ::std::os::raw::c_void,
        ) -> bool;
    }
}

mod internal {
    #[cfg(feature = "pg10")]
    pub use pg10::*;

    #[cfg(feature = "pg11")]
    pub use pg11::*;

    #[cfg(feature = "pg12")]
    pub use pg12::*;

    //
    // for specific versions
    //

    #[cfg(feature = "pg10")]
    mod pg10 {
        pub use crate::common::*;
        pub use crate::pg10_specific::*;

        pub use crate::pg10_specific::tupleDesc as TupleDescData;
        pub use crate::pg10_specific::AllocSetContextCreate as AllocSetContextCreateExtended;

        pub unsafe fn add_string_reloption(
            kinds: bits32,
            name: *const ::std::os::raw::c_char,
            desc: *const ::std::os::raw::c_char,
            default_val: *const ::std::os::raw::c_char,
            validator: ::std::option::Option<
                unsafe extern "C" fn(value: *const ::std::os::raw::c_char),
            >,
        ) {
            // PG10 defines the validator function as taking a "*mut c_char"
            // whereas PG11/12 want a "*const c_char".
            //
            // For ease of use by users of this crate, we cast the provided
            // 'validator' function to what PG10 wants, using transmute
            //
            // If there's a better way to do this, I'ld love to know!
            let func_as_mut_arg = match validator {
                Some(func) => {
                    let func_ptr = std::mem::transmute::<
                        unsafe extern "C" fn(*const ::std::os::raw::c_char),
                        unsafe extern "C" fn(*mut ::std::os::raw::c_char),
                    >(func);
                    Some(func_ptr)
                }
                None => None,
            };

            crate::pg10_specific::add_string_reloption(
                kinds,
                name as *mut std::os::raw::c_char,
                desc as *mut std::os::raw::c_char,
                default_val as *mut std::os::raw::c_char,
                func_as_mut_arg,
            );
        }

        pub unsafe fn add_int_reloption(
            kinds: bits32,
            name: *const ::std::os::raw::c_char,
            desc: *const ::std::os::raw::c_char,
            default_val: ::std::os::raw::c_int,
            min_val: ::std::os::raw::c_int,
            max_val: ::std::os::raw::c_int,
        ) {
            crate::pg10_specific::add_int_reloption(
                kinds,
                name as *mut std::os::raw::c_char,
                desc as *mut std::os::raw::c_char,
                default_val,
                min_val,
                max_val,
            );
        }

        pub unsafe fn add_bool_reloption(
            kinds: bits32,
            name: *const ::std::os::raw::c_char,
            desc: *const ::std::os::raw::c_char,
            default_val: bool,
        ) {
            crate::pg10_specific::add_bool_reloption(
                kinds,
                name as *mut std::os::raw::c_char,
                desc as *mut std::os::raw::c_char,
                default_val,
            );
        }

        /// # Safety
        ///
        /// This function wraps Postgres' internal `IndexBuildHeapScan` method, and therfore, is
        /// inherently unsafe
        pub unsafe fn IndexBuildHeapScan<T>(
            heap_relation: crate::Relation,
            index_relation: crate::Relation,
            index_info: *mut crate::pg10_specific::IndexInfo,
            build_callback: crate::IndexBuildCallback,
            build_callback_state: *mut T,
        ) {
            crate::pg10_specific::IndexBuildHeapScan(
                heap_relation,
                index_relation,
                index_info,
                true,
                build_callback,
                build_callback_state as *mut std::os::raw::c_void,
            );
        }
    }

    #[cfg(feature = "pg11")]
    mod pg11 {
        pub use crate::common::*;
        pub use crate::pg11_specific::*;

        pub use crate::pg11_specific::tupleDesc as TupleDescData;

        /// # Safety
        ///
        /// This function wraps Postgres' internal `IndexBuildHeapScan` method, and therfore, is
        /// inherently unsafe
        pub unsafe fn IndexBuildHeapScan<T>(
            heap_relation: crate::Relation,
            index_relation: crate::Relation,
            index_info: *mut crate::pg11_specific::IndexInfo,
            build_callback: crate::IndexBuildCallback,
            build_callback_state: *mut T,
        ) {
            crate::pg11_specific::IndexBuildHeapScan(
                heap_relation,
                index_relation,
                index_info,
                true,
                build_callback,
                build_callback_state as *mut std::os::raw::c_void,
                std::ptr::null_mut(),
            );
        }
    }

    #[cfg(feature = "pg12")]
    mod pg12 {
        pub use crate::common::*;
        pub use crate::pg12_specific::*;

        pub use crate::pg12_specific::AllocSetContextCreateInternal as AllocSetContextCreateExtended;

        pub const QTW_EXAMINE_RTES: u32 = crate::pg12_specific::QTW_EXAMINE_RTES_BEFORE;

        /// # Safety
        ///
        /// This function wraps Postgres' internal `IndexBuildHeapScan` method, and therfore, is
        /// inherently unsafe
        pub unsafe fn IndexBuildHeapScan<T>(
            heap_relation: crate::Relation,
            index_relation: crate::Relation,
            index_info: *mut crate::pg12_specific::IndexInfo,
            build_callback: crate::IndexBuildCallback,
            build_callback_state: *mut T,
        ) {
            let heap_relation_ref = heap_relation.as_ref().unwrap();
            let table_am = heap_relation_ref.rd_tableam.as_ref().unwrap();

            table_am.index_build_range_scan.unwrap()(
                heap_relation,
                index_relation,
                index_info,
                true,
                false,
                true,
                0,
                crate::InvalidBlockNumber,
                build_callback,
                build_callback_state as *mut std::os::raw::c_void,
                std::ptr::null_mut(),
            );
        }
    }
}
