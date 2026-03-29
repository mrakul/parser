use crate::error::ParserError;

pub trait CsvFormatIO<InternalType> {
    /// Чтение из любого приёмника, реализующего трейт Read (без &self для создания экземпляра "класса")
    fn new_from_csv_reader<R: std::io::BufRead>(reader: &mut R) -> Result<InternalType, ParserError>;

    /// Запись в любой приёмник, реализующий трейт Write
    fn write_as_csv_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>;
}