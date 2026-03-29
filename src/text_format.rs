use crate::error::ParserError;

pub trait TextFormatIO<InternalType> {
    // Читает отчёт из любого приёмника, реализующего трейт Read (статическая, для создания экземпляра "класса")
    fn new_from_text_reader<R: std::io::BufRead>(reader: &mut R) -> Result<InternalType, ParserError>;

    /// Запись в любой приёмник, реализующий трейт Write
    fn write_as_text_to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError>;
}