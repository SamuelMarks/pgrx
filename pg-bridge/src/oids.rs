#![allow(non_camel_case_types)]
use crate::pg_sys;

pub enum PgOid {
    Custom(pg_sys::Oid),
    CommonBuiltIn(CommonBuiltInOids),

    #[cfg(feature = "pg10")]
    BuiltInPg10(Pg10BuiltInOids),

    #[cfg(feature = "pg11")]
    BuiltInPg11(Pg11BuiltInOids),

    #[cfg(feature = "pg12")]
    BuiltInPg12(Pg12BuiltInOids),
}

impl PgOid {
    pub fn value(self) -> pg_sys::Oid {
        match self {
            PgOid::Custom(val) => val as pg_sys::Oid,
            PgOid::CommonBuiltIn(oid) => oid as pg_sys::Oid,

            #[cfg(feature = "pg10")]
            PgOid::BuiltInPg10(oid) => oid as pg_sys::Oid,

            #[cfg(feature = "pg11")]
            PgOid::BuiltInPg11(oid) => oid as pg_sys::Oid,

            #[cfg(feature = "pg12")]
            PgOid::BuiltInPg12(oid) => oid as pg_sys::Oid,
        }
    }
}

#[cfg(feature = "pg10")]
pub enum Pg10BuiltInOids {
    ABSTIMEOID = pg_sys::pg10_specific::ABSTIMEOID as isize,
    RELTIMEOID = pg_sys::pg10_specific::RELTIMEOID as isize,
    TINTERVALOID = pg_sys::pg10_specific::TINTERVALOID as isize,
}

#[cfg(feature = "pg11")]
pub enum Pg11BuiltInOids {
    XMLARRAYOID = pg_sys::pg11_specific::XMLARRAYOID as isize,
    TSVECTORARRAYOID = pg_sys::pg11_specific::TSVECTORARRAYOID as isize,
    GTSVECTORARRAYOID = pg_sys::pg11_specific::GTSVECTORARRAYOID as isize,
    TSQUERYARRAYOID = pg_sys::pg11_specific::TSQUERYARRAYOID as isize,
    REGCONFIGARRAYOID = pg_sys::pg11_specific::REGCONFIGARRAYOID as isize,
    REGDICTIONARYARRAYOID = pg_sys::pg11_specific::REGDICTIONARYARRAYOID as isize,
    JSONBARRAYOID = pg_sys::pg11_specific::JSONBARRAYOID as isize,
    TXID_SNAPSHOTOID = pg_sys::pg11_specific::TXID_SNAPSHOTOID as isize,
    TXID_SNAPSHOTARRAYOID = pg_sys::pg11_specific::TXID_SNAPSHOTARRAYOID as isize,
    INT4RANGEARRAYOID = pg_sys::pg11_specific::INT4RANGEARRAYOID as isize,
    NUMRANGEOID = pg_sys::pg11_specific::NUMRANGEOID as isize,
    NUMRANGEARRAYOID = pg_sys::pg11_specific::NUMRANGEARRAYOID as isize,
    TSRANGEOID = pg_sys::pg11_specific::TSRANGEOID as isize,
    TSRANGEARRAYOID = pg_sys::pg11_specific::TSRANGEARRAYOID as isize,
    TSTZRANGEOID = pg_sys::pg11_specific::TSTZRANGEOID as isize,
    TSTZRANGEARRAYOID = pg_sys::pg11_specific::TSTZRANGEARRAYOID as isize,
    DATERANGEOID = pg_sys::pg11_specific::DATERANGEOID as isize,
    DATERANGEARRAYOID = pg_sys::pg11_specific::DATERANGEARRAYOID as isize,
    INT8RANGEOID = pg_sys::pg11_specific::INT8RANGEOID as isize,
    INT8RANGEARRAYOID = pg_sys::pg11_specific::INT8RANGEARRAYOID as isize,
    JSONARRAYOID = pg_sys::pg11_specific::JSONARRAYOID as isize,
    SMGROID = pg_sys::pg11_specific::SMGROID as isize,
    LINEARRAYOID = pg_sys::pg11_specific::LINEARRAYOID as isize,
    ABSTIMEOID = pg_sys::pg11_specific::ABSTIMEOID as isize,
    RELTIMEOID = pg_sys::pg11_specific::RELTIMEOID as isize,
    TINTERVALOID = pg_sys::pg11_specific::TINTERVALOID as isize,
    CIRCLEARRAYOID = pg_sys::pg11_specific::CIRCLEARRAYOID as isize,
    MONEYARRAYOID = pg_sys::pg11_specific::MONEYARRAYOID as isize,
    BOOLARRAYOID = pg_sys::pg11_specific::BOOLARRAYOID as isize,
    BYTEAARRAYOID = pg_sys::pg11_specific::BYTEAARRAYOID as isize,
    CHARARRAYOID = pg_sys::pg11_specific::CHARARRAYOID as isize,
    NAMEARRAYOID = pg_sys::pg11_specific::NAMEARRAYOID as isize,
    INT2VECTORARRAYOID = pg_sys::pg11_specific::INT2VECTORARRAYOID as isize,
    REGPROCARRAYOID = pg_sys::pg11_specific::REGPROCARRAYOID as isize,
    TIDARRAYOID = pg_sys::pg11_specific::TIDARRAYOID as isize,
    XIDARRAYOID = pg_sys::pg11_specific::XIDARRAYOID as isize,
    CIDARRAYOID = pg_sys::pg11_specific::CIDARRAYOID as isize,
    OIDVECTORARRAYOID = pg_sys::pg11_specific::OIDVECTORARRAYOID as isize,
    BPCHARARRAYOID = pg_sys::pg11_specific::BPCHARARRAYOID as isize,
    VARCHARARRAYOID = pg_sys::pg11_specific::VARCHARARRAYOID as isize,
    INT8ARRAYOID = pg_sys::pg11_specific::INT8ARRAYOID as isize,
    POINTARRAYOID = pg_sys::pg11_specific::POINTARRAYOID as isize,
    LSEGARRAYOID = pg_sys::pg11_specific::LSEGARRAYOID as isize,
    PATHARRAYOID = pg_sys::pg11_specific::PATHARRAYOID as isize,
    BOXARRAYOID = pg_sys::pg11_specific::BOXARRAYOID as isize,
    FLOAT8ARRAYOID = pg_sys::pg11_specific::FLOAT8ARRAYOID as isize,
    ABSTIMEARRAYOID = pg_sys::pg11_specific::ABSTIMEARRAYOID as isize,
    RELTIMEARRAYOID = pg_sys::pg11_specific::RELTIMEARRAYOID as isize,
    TINTERVALARRAYOID = pg_sys::pg11_specific::TINTERVALARRAYOID as isize,
    POLYGONARRAYOID = pg_sys::pg11_specific::POLYGONARRAYOID as isize,
    ACLITEMARRAYOID = pg_sys::pg11_specific::ACLITEMARRAYOID as isize,
    MACADDRARRAYOID = pg_sys::pg11_specific::MACADDRARRAYOID as isize,
    MACADDR8ARRAYOID = pg_sys::pg11_specific::MACADDR8ARRAYOID as isize,
    INETARRAYOID = pg_sys::pg11_specific::INETARRAYOID as isize,
    CIDRARRAYOID = pg_sys::pg11_specific::CIDRARRAYOID as isize,
    TIMESTAMPARRAYOID = pg_sys::pg11_specific::TIMESTAMPARRAYOID as isize,
    DATEARRAYOID = pg_sys::pg11_specific::DATEARRAYOID as isize,
    TIMEARRAYOID = pg_sys::pg11_specific::TIMEARRAYOID as isize,
    REFCURSORARRAYOID = pg_sys::pg11_specific::REFCURSORARRAYOID as isize,
    VARBITARRAYOID = pg_sys::pg11_specific::VARBITARRAYOID as isize,
    BITARRAYOID = pg_sys::pg11_specific::BITARRAYOID as isize,
    TIMETZARRAYOID = pg_sys::pg11_specific::TIMETZARRAYOID as isize,
    TIMESTAMPTZARRAYOID = pg_sys::pg11_specific::TIMESTAMPTZARRAYOID as isize,
    INTERVALARRAYOID = pg_sys::pg11_specific::INTERVALARRAYOID as isize,
    NUMERICARRAYOID = pg_sys::pg11_specific::NUMERICARRAYOID as isize,
    UUIDARRAYOID = pg_sys::pg11_specific::UUIDARRAYOID as isize,
    REGPROCEDUREARRAYOID = pg_sys::pg11_specific::REGPROCEDUREARRAYOID as isize,
    REGOPERARRAYOID = pg_sys::pg11_specific::REGOPERARRAYOID as isize,
    REGOPERATORARRAYOID = pg_sys::pg11_specific::REGOPERATORARRAYOID as isize,
    REGCLASSARRAYOID = pg_sys::pg11_specific::REGCLASSARRAYOID as isize,
    REGROLEARRAYOID = pg_sys::pg11_specific::REGROLEARRAYOID as isize,
    REGNAMESPACEARRAYOID = pg_sys::pg11_specific::REGNAMESPACEARRAYOID as isize,
    PG_LSNARRAYOID = pg_sys::pg11_specific::PG_LSNARRAYOID as isize,
}

#[cfg(feature = "pg12")]
pub enum Pg12BuiltInOids {
    XMLARRAYOID = pg_sys::pg12_specific::XMLARRAYOID as isize,
    TSVECTORARRAYOID = pg_sys::pg12_specific::TSVECTORARRAYOID as isize,
    GTSVECTORARRAYOID = pg_sys::pg12_specific::GTSVECTORARRAYOID as isize,
    TSQUERYARRAYOID = pg_sys::pg12_specific::TSQUERYARRAYOID as isize,
    REGCONFIGARRAYOID = pg_sys::pg12_specific::REGCONFIGARRAYOID as isize,
    REGDICTIONARYARRAYOID = pg_sys::pg12_specific::REGDICTIONARYARRAYOID as isize,
    JSONBARRAYOID = pg_sys::pg12_specific::JSONBARRAYOID as isize,
    TXID_SNAPSHOTOID = pg_sys::pg12_specific::TXID_SNAPSHOTOID as isize,
    TXID_SNAPSHOTARRAYOID = pg_sys::pg12_specific::TXID_SNAPSHOTARRAYOID as isize,
    INT4RANGEARRAYOID = pg_sys::pg12_specific::INT4RANGEARRAYOID as isize,
    NUMRANGEOID = pg_sys::pg12_specific::NUMRANGEOID as isize,
    NUMRANGEARRAYOID = pg_sys::pg12_specific::NUMRANGEARRAYOID as isize,
    TSRANGEOID = pg_sys::pg12_specific::TSRANGEOID as isize,
    TSRANGEARRAYOID = pg_sys::pg12_specific::TSRANGEARRAYOID as isize,
    TSTZRANGEOID = pg_sys::pg12_specific::TSTZRANGEOID as isize,
    TSTZRANGEARRAYOID = pg_sys::pg12_specific::TSTZRANGEARRAYOID as isize,
    DATERANGEOID = pg_sys::pg12_specific::DATERANGEOID as isize,
    DATERANGEARRAYOID = pg_sys::pg12_specific::DATERANGEARRAYOID as isize,
    INT8RANGEOID = pg_sys::pg12_specific::INT8RANGEOID as isize,
    INT8RANGEARRAYOID = pg_sys::pg12_specific::INT8RANGEARRAYOID as isize,
    JSONARRAYOID = pg_sys::pg12_specific::JSONARRAYOID as isize,
    LINEARRAYOID = pg_sys::pg12_specific::LINEARRAYOID as isize,
    CIRCLEARRAYOID = pg_sys::pg12_specific::CIRCLEARRAYOID as isize,
    MONEYARRAYOID = pg_sys::pg12_specific::MONEYARRAYOID as isize,
    BOOLARRAYOID = pg_sys::pg12_specific::BOOLARRAYOID as isize,
    BYTEAARRAYOID = pg_sys::pg12_specific::BYTEAARRAYOID as isize,
    CHARARRAYOID = pg_sys::pg12_specific::CHARARRAYOID as isize,
    NAMEARRAYOID = pg_sys::pg12_specific::NAMEARRAYOID as isize,
    INT2VECTORARRAYOID = pg_sys::pg12_specific::INT2VECTORARRAYOID as isize,
    REGPROCARRAYOID = pg_sys::pg12_specific::REGPROCARRAYOID as isize,
    TIDARRAYOID = pg_sys::pg12_specific::TIDARRAYOID as isize,
    XIDARRAYOID = pg_sys::pg12_specific::XIDARRAYOID as isize,
    CIDARRAYOID = pg_sys::pg12_specific::CIDARRAYOID as isize,
    OIDVECTORARRAYOID = pg_sys::pg12_specific::OIDVECTORARRAYOID as isize,
    BPCHARARRAYOID = pg_sys::pg12_specific::BPCHARARRAYOID as isize,
    VARCHARARRAYOID = pg_sys::pg12_specific::VARCHARARRAYOID as isize,
    INT8ARRAYOID = pg_sys::pg12_specific::INT8ARRAYOID as isize,
    POINTARRAYOID = pg_sys::pg12_specific::POINTARRAYOID as isize,
    LSEGARRAYOID = pg_sys::pg12_specific::LSEGARRAYOID as isize,
    PATHARRAYOID = pg_sys::pg12_specific::PATHARRAYOID as isize,
    BOXARRAYOID = pg_sys::pg12_specific::BOXARRAYOID as isize,
    FLOAT8ARRAYOID = pg_sys::pg12_specific::FLOAT8ARRAYOID as isize,
    POLYGONARRAYOID = pg_sys::pg12_specific::POLYGONARRAYOID as isize,
    ACLITEMARRAYOID = pg_sys::pg12_specific::ACLITEMARRAYOID as isize,
    MACADDRARRAYOID = pg_sys::pg12_specific::MACADDRARRAYOID as isize,
    MACADDR8ARRAYOID = pg_sys::pg12_specific::MACADDR8ARRAYOID as isize,
    INETARRAYOID = pg_sys::pg12_specific::INETARRAYOID as isize,
    CIDRARRAYOID = pg_sys::pg12_specific::CIDRARRAYOID as isize,
    TIMESTAMPARRAYOID = pg_sys::pg12_specific::TIMESTAMPARRAYOID as isize,
    DATEARRAYOID = pg_sys::pg12_specific::DATEARRAYOID as isize,
    TIMEARRAYOID = pg_sys::pg12_specific::TIMEARRAYOID as isize,
    REFCURSORARRAYOID = pg_sys::pg12_specific::REFCURSORARRAYOID as isize,
    VARBITARRAYOID = pg_sys::pg12_specific::VARBITARRAYOID as isize,
    BITARRAYOID = pg_sys::pg12_specific::BITARRAYOID as isize,
    TIMETZARRAYOID = pg_sys::pg12_specific::TIMETZARRAYOID as isize,
    TIMESTAMPTZARRAYOID = pg_sys::pg12_specific::TIMESTAMPTZARRAYOID as isize,
    INTERVALARRAYOID = pg_sys::pg12_specific::INTERVALARRAYOID as isize,
    NUMERICARRAYOID = pg_sys::pg12_specific::NUMERICARRAYOID as isize,
    UUIDARRAYOID = pg_sys::pg12_specific::UUIDARRAYOID as isize,
    REGPROCEDUREARRAYOID = pg_sys::pg12_specific::REGPROCEDUREARRAYOID as isize,
    REGOPERARRAYOID = pg_sys::pg12_specific::REGOPERARRAYOID as isize,
    REGOPERATORARRAYOID = pg_sys::pg12_specific::REGOPERATORARRAYOID as isize,
    REGCLASSARRAYOID = pg_sys::pg12_specific::REGCLASSARRAYOID as isize,
    REGROLEARRAYOID = pg_sys::pg12_specific::REGROLEARRAYOID as isize,
    REGNAMESPACEARRAYOID = pg_sys::pg12_specific::REGNAMESPACEARRAYOID as isize,
    PG_LSNARRAYOID = pg_sys::pg12_specific::PG_LSNARRAYOID as isize,
}

pub enum CommonBuiltInOids {
    BOOLOID = pg_sys::BOOLOID as isize,
    BYTEAOID = pg_sys::BYTEAOID as isize,
    CHAROID = pg_sys::CHAROID as isize,
    NAMEOID = pg_sys::NAMEOID as isize,
    INT8OID = pg_sys::INT8OID as isize,
    INT2OID = pg_sys::INT2OID as isize,
    INT2VECTOROID = pg_sys::INT2VECTOROID as isize,
    INT4OID = pg_sys::INT4OID as isize,
    REGPROCOID = pg_sys::REGPROCOID as isize,
    TEXTOID = pg_sys::TEXTOID as isize,
    OIDOID = pg_sys::OIDOID as isize,
    TIDOID = pg_sys::TIDOID as isize,
    XIDOID = pg_sys::XIDOID as isize,
    CIDOID = pg_sys::CIDOID as isize,
    OIDVECTOROID = pg_sys::OIDVECTOROID as isize,
    JSONOID = pg_sys::JSONOID as isize,
    XMLOID = pg_sys::XMLOID as isize,
    PGNODETREEOID = pg_sys::PGNODETREEOID as isize,
    PGNDISTINCTOID = pg_sys::PGNDISTINCTOID as isize,
    PGDEPENDENCIESOID = pg_sys::PGDEPENDENCIESOID as isize,
    PGDDLCOMMANDOID = pg_sys::PGDDLCOMMANDOID as isize,
    POINTOID = pg_sys::POINTOID as isize,
    LSEGOID = pg_sys::LSEGOID as isize,
    PATHOID = pg_sys::PATHOID as isize,
    BOXOID = pg_sys::BOXOID as isize,
    POLYGONOID = pg_sys::POLYGONOID as isize,
    LINEOID = pg_sys::LINEOID as isize,
    FLOAT4OID = pg_sys::FLOAT4OID as isize,
    FLOAT8OID = pg_sys::FLOAT8OID as isize,
    UNKNOWNOID = pg_sys::UNKNOWNOID as isize,
    CIRCLEOID = pg_sys::CIRCLEOID as isize,
    CASHOID = pg_sys::CASHOID as isize,
    MACADDROID = pg_sys::MACADDROID as isize,
    INETOID = pg_sys::INETOID as isize,
    CIDROID = pg_sys::CIDROID as isize,
    MACADDR8OID = pg_sys::MACADDR8OID as isize,
    INT2ARRAYOID = pg_sys::INT2ARRAYOID as isize,
    INT4ARRAYOID = pg_sys::INT4ARRAYOID as isize,
    TEXTARRAYOID = pg_sys::TEXTARRAYOID as isize,
    OIDARRAYOID = pg_sys::OIDARRAYOID as isize,
    FLOAT4ARRAYOID = pg_sys::FLOAT4ARRAYOID as isize,
    ACLITEMOID = pg_sys::ACLITEMOID as isize,
    CSTRINGARRAYOID = pg_sys::CSTRINGARRAYOID as isize,
    BPCHAROID = pg_sys::BPCHAROID as isize,
    VARCHAROID = pg_sys::VARCHAROID as isize,
    DATEOID = pg_sys::DATEOID as isize,
    TIMEOID = pg_sys::TIMEOID as isize,
    TIMESTAMPOID = pg_sys::TIMESTAMPOID as isize,
    TIMESTAMPTZOID = pg_sys::TIMESTAMPTZOID as isize,
    INTERVALOID = pg_sys::INTERVALOID as isize,
    TIMETZOID = pg_sys::TIMETZOID as isize,
    BITOID = pg_sys::BITOID as isize,
    VARBITOID = pg_sys::VARBITOID as isize,
    NUMERICOID = pg_sys::NUMERICOID as isize,
    REFCURSOROID = pg_sys::REFCURSOROID as isize,
    REGPROCEDUREOID = pg_sys::REGPROCEDUREOID as isize,
    REGOPEROID = pg_sys::REGOPEROID as isize,
    REGOPERATOROID = pg_sys::REGOPERATOROID as isize,
    REGCLASSOID = pg_sys::REGCLASSOID as isize,
    REGTYPEOID = pg_sys::REGTYPEOID as isize,
    REGROLEOID = pg_sys::REGROLEOID as isize,
    REGNAMESPACEOID = pg_sys::REGNAMESPACEOID as isize,
    REGTYPEARRAYOID = pg_sys::REGTYPEARRAYOID as isize,
    UUIDOID = pg_sys::UUIDOID as isize,
    LSNOID = pg_sys::LSNOID as isize,
    TSVECTOROID = pg_sys::TSVECTOROID as isize,
    GTSVECTOROID = pg_sys::GTSVECTOROID as isize,
    TSQUERYOID = pg_sys::TSQUERYOID as isize,
    REGCONFIGOID = pg_sys::REGCONFIGOID as isize,
    REGDICTIONARYOID = pg_sys::REGDICTIONARYOID as isize,
    JSONBOID = pg_sys::JSONBOID as isize,
    INT4RANGEOID = pg_sys::INT4RANGEOID as isize,
    RECORDOID = pg_sys::RECORDOID as isize,
    RECORDARRAYOID = pg_sys::RECORDARRAYOID as isize,
    CSTRINGOID = pg_sys::CSTRINGOID as isize,
    ANYOID = pg_sys::ANYOID as isize,
    ANYARRAYOID = pg_sys::ANYARRAYOID as isize,
    VOIDOID = pg_sys::VOIDOID as isize,
    TRIGGEROID = pg_sys::TRIGGEROID as isize,
    EVTTRIGGEROID = pg_sys::EVTTRIGGEROID as isize,
    LANGUAGE_HANDLEROID = pg_sys::LANGUAGE_HANDLEROID as isize,
    INTERNALOID = pg_sys::INTERNALOID as isize,
    OPAQUEOID = pg_sys::OPAQUEOID as isize,
    ANYELEMENTOID = pg_sys::ANYELEMENTOID as isize,
    ANYNONARRAYOID = pg_sys::ANYNONARRAYOID as isize,
    ANYENUMOID = pg_sys::ANYENUMOID as isize,
    FDW_HANDLEROID = pg_sys::FDW_HANDLEROID as isize,
    INDEX_AM_HANDLEROID = pg_sys::INDEX_AM_HANDLEROID as isize,
    TSM_HANDLEROID = pg_sys::TSM_HANDLEROID as isize,
    ANYRANGEOID = pg_sys::ANYRANGEOID as isize,
}
