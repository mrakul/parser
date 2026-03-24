use crate::error::ParserError;

pub trait TextFormatIO<InternalType> {
    // Читает отчёт из любого приёмника, реализующего трейт Read (статическая, для создания экземпляра "класса")
    fn new_from_text_file<R: std::io::Read>(reader: R) -> Result<InternalType, ParserError>;

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to_text_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), ParserError>;
}