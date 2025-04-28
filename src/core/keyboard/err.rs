#[derive(Debug)]
pub enum ListenError {
	AlreadyInit,
	MissingDisplay,
	RecordContextEnabling,
	RecordContext,
	XRecordExtension,
}
