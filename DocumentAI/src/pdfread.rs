use lopdf::Document;
use rustring_builder::StringBuilder;
pub fn read_all_pdf(path: &str) -> String {
    let document = Document::load(path);
    let mut sb = StringBuilder::new();
    match document {
        Ok(doc)=>{
            let pages = doc.get_pages();
            for (i, _) in pages.iter().enumerate(){
                let text = doc.extract_text(&[i.try_into().unwrap()]);
                sb.append(text.unwrap_or_default());
            }
        }
        Err(err) => {}
    }
    sb.to_string()
    
}