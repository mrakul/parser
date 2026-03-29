use crate::error::ParserError;
pub trait BinFormatIO<InternalType> {
    /// Чтение из любого приёмника, реализующего трейт Read (без &self для создания экземпляра "класса")
    fn new_from_bin_reader<R: std::io::Read>(reader: &mut R) -> Result<InternalType, ParserError>;

    /// Запись в любой приёмник, реализующий трейт Write
    fn write_as_bin_to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ParserError>;
}