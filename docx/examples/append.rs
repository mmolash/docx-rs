use docx::{document::Paragraph, Docx, DocxFile, Result};

fn main() -> Result<()> {
    let mut docx = Docx::default();

    let mut para = Paragraph::default();
    para.text("hello, world");
    docx.insert_para(para);

    docx.write_file("origin.docx")?;

    let docx = DocxFile::from_file("origin.docx")?;
    let mut docx = docx.parse()?;

    let mut para = Paragraph::default();
    para.text("world, hello");
    docx.insert_para(para);

    docx.write_file("origin_appended.docx")?;

    Ok(())
}
