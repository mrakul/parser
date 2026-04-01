use crate::error::ParserError;
pub trait BinFormatIO<InternalType> {
    /// При реализации - получение структуры InternalType из BIN-представления
    /// Читает из любого приёмника, реализующего трейт Read
    /// Входной параметр: <R: std::io::Read>(reader: &mut R)
    /// Если неуспешно => ошибка ParserError, описаны в error.rs
    fn new_from_bin_reader<R: std::io::Read>(reader: &mut R) -> Result<InternalType, ParserError>;

    /// При реализции - запись структуры Self в BIN-формат в передаваемый writer
    /// Запись возможна в любой приёмник, реализующий трейт Write
    /// Входной параметр: <W: std::io::Write>(&self, writer: &mut W) (вызывается от экземпляра структуры)
    /// Если неуспешно => ошибка ParserError, описаны в error.rs
    fn write_as_bin_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>;
}