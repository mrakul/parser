pub trait CsvFormatIO<InternalType> {
    // Читает отчёт из любого приёмника, реализующего трейт Read (статическая, для создания экземпляра "класса")
    fn new_from_csv_file<R: std::io::Read>(reader: &mut R) -> Result<InternalType, String>;

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to_csv_file<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), String>;

    // fn read_from_csv_file<R: std::io::Read>(&self, r: &mut R) -> Result<&InternalType, String> {
    //     todo!()
    // }
}