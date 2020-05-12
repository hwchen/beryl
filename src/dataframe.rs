#[derive(Debug, Clone)]
pub struct DataFrame {
    pub columns: Vec<Column>,
}

impl DataFrame {
    pub fn new() -> Self {
        DataFrame {
            columns: vec![],
        }
    }

    pub fn from_vec(columns: Vec<Column>) -> Self {
        DataFrame {
            columns
        }
    }

    pub fn len(&self) -> usize {
        if let Some(col) = self.columns.get(0) {
            match col.column_data {
                ColumnData::Int8(ref ns) => ns.len(),
                ColumnData::Int16(ref ns) => ns.len(),
                ColumnData::Int32(ref ns) => ns.len(),
                ColumnData::Int64(ref ns) => ns.len(),
                ColumnData::UInt8(ref ns) => ns.len(),
                ColumnData::UInt16(ref ns) => ns.len(),
                ColumnData::UInt32(ref ns) => ns.len(),
                ColumnData::UInt64(ref ns) => ns.len(),
                ColumnData::Float32(ref ns) => ns.len(),
                ColumnData::Float64(ref ns) => ns.len(),
                ColumnData::Text(ref ss) => ss.len(),
                ColumnData::NullableInt8(ref ns) => ns.len(),
                ColumnData::NullableInt16(ref ns) => ns.len(),
                ColumnData::NullableInt32(ref ns) => ns.len(),
                ColumnData::NullableInt64(ref ns) => ns.len(),
                ColumnData::NullableUInt8(ref ns) => ns.len(),
                ColumnData::NullableUInt16(ref ns) => ns.len(),
                ColumnData::NullableUInt32(ref ns) => ns.len(),
                ColumnData::NullableUInt64(ref ns) => ns.len(),
                ColumnData::NullableFloat32(ref ns) => ns.len(),
                ColumnData::NullableFloat64(ref ns) => ns.len(),
                ColumnData::NullableText(ref ss) => ss.len(),
                ColumnData::ArrayInt8(ref ns) => ns.len(),
                ColumnData::ArrayInt16(ref ns) => ns.len(),
                ColumnData::ArrayInt32(ref ns) => ns.len(),
                ColumnData::ArrayInt64(ref ns) => ns.len(),
                ColumnData::ArrayUInt8(ref ns) => ns.len(),
                ColumnData::ArrayUInt16(ref ns) => ns.len(),
                ColumnData::ArrayUInt32(ref ns) => ns.len(),
                ColumnData::ArrayUInt64(ref ns) => ns.len(),
                ColumnData::ArrayFloat32(ref ns) => ns.len(),
                ColumnData::ArrayFloat64(ref ns) => ns.len(),
                ColumnData::ArrayText(ref ss) => ss.len(),
            }
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub column_data: ColumnData,
}

impl Column {
    pub fn new(name: String, column_data: ColumnData) -> Self {
        Column {
            name,
            column_data,
        }
    }

    pub fn column_data(&mut self) ->&mut ColumnData {
        &mut self.column_data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnData {
    Int8(Vec<i8>),
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    UInt8(Vec<u8>),
    UInt16(Vec<u16>),
    UInt32(Vec<u32>),
    UInt64(Vec<u64>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
    Text(Vec<String>),
    NullableInt8(Vec<Option<i8>>),
    NullableInt16(Vec<Option<i16>>),
    NullableInt32(Vec<Option<i32>>),
    NullableInt64(Vec<Option<i64>>),
    NullableUInt8(Vec<Option<u8>>),
    NullableUInt16(Vec<Option<u16>>),
    NullableUInt32(Vec<Option<u32>>),
    NullableUInt64(Vec<Option<u64>>),
    NullableFloat32(Vec<Option<f32>>),
    NullableFloat64(Vec<Option<f64>>),
    NullableText(Vec<Option<String>>),
    ArrayInt8(Vec<Vec<i8>>),
    ArrayInt16(Vec<Vec<i16>>),
    ArrayInt32(Vec<Vec<i32>>),
    ArrayInt64(Vec<Vec<i64>>),
    ArrayUInt8(Vec<Vec<u8>>),
    ArrayUInt16(Vec<Vec<u16>>),
    ArrayUInt32(Vec<Vec<u32>>),
    ArrayUInt64(Vec<Vec<u64>>),
    ArrayFloat32(Vec<Vec<f32>>),
    ArrayFloat64(Vec<Vec<f64>>),
    ArrayText(Vec<Vec<String>>)
}
