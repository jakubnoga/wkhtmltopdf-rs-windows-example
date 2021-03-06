use wkhtmltopdf::*;

fn main() {
    let html = r#"<html><body><div>foo</div></body></html>"#;
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app
        .builder()
        .orientation(Orientation::Landscape)
        .margin(Size::Inches(2))
        .title("Awesome Foo")
        .build_from_html(&html)
        .expect("failed to build pdf");

    pdfout.save("foo.pdf").expect("failed to save foo.pdf");
    println!("generated PDF saved as: foo.pdf");
}
