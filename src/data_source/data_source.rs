use sciter::Value;

pub trait DataSource {
	fn data_source_columns(&self) -> Value;
	fn data_source_row_count(&self) -> i32;
	fn data_source_rows_data(&self, row_index: i32, row_count: i32) -> Value;
}
