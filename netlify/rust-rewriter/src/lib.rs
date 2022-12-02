use wasm_bindgen::prelude::*;
use chrono::prelude::*;
extern crate kuchiki;
use kuchiki::traits::*;

fn should_remove_header(show_until: &str, is_mainmatter: bool) -> bool {
  match Utc.datetime_from_str(show_until, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => {
      let now = Utc::now();
      let is_in_past = date < now;
      is_in_past || is_mainmatter
    },
    Err(_)=> false,
  }
}

fn remove_element(id: &str, html: &str) -> String {
  let document = kuchiki::parse_html().one(html);

  match document.select_first(id) {
    Ok(element) => {
      element.as_node().detach();
      document.to_string()
    },
    Err(_) => {
      html.to_string()
    },
  }
  
}

#[wasm_bindgen]
pub fn remove_header(response_text: &str, selector_to_remove: &str, show_until: &str, is_mainmatter: bool) -> String {
  if should_remove_header(show_until, is_mainmatter) {
    return remove_element(selector_to_remove, response_text);
  }
  return response_text.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_date(days: i64) -> String {
      let now = Utc::now();
      let date = now + chrono::Duration::days(days);
      date.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[test]
    fn do_nothing_when_wrong_date(){
      let wrong_date = "1.1.asdfasdf";
      let result = should_remove_header(wrong_date, true);
      assert_eq!(result, false);
    }

    #[test]
    fn it_remove_header_when_mainmatter_future(){
      let date = get_date(2);
      let result = should_remove_header(&date, true);
      assert_eq!(result, true);
    }

    #[test]
    fn it_remove_header_when_mainmatter_past(){
      let date = get_date(-2);
      let result = should_remove_header(&date, true);
      assert_eq!(result, true);
    }

    #[test]
    fn it_remove_header_when_simplabs_past(){
      let date = get_date(-2);
      let result = should_remove_header(&date, false);
      assert_eq!(result, true);
    }

    #[test]
    fn it_doesnotremove_header_when_simplabs_future(){
      let date = get_date(2);
      let result = should_remove_header(&date, false);
      assert_eq!(result, false);
    }

    #[test]
    fn it_rewrites_request() {
      let response_text = "<html><head></head><body><div id='simplabsheader'>simplabs is Mainmatter now!</div></body></html>";
      let result = remove_element("#simplabsheader", response_text);
      assert_eq!(result, "<html><head></head><body></body></html>");
    }

    #[test]
    fn it_do_nothing_if_element_doesnotexist() {
      let response_text = "<html><head></head><body></body></html>";
      let result = remove_element("#simplabsheader", response_text);
      assert_eq!(result, "<html><head></head><body></body></html>");
    }

    #[test]
    fn it_rewrites_request1() {
      let response_text = "<!DOCTYPE html><html><head></head><body><div id='simplabsheader'>Simplabs is Mainmatter (should be removed)</div> Some content </body></html>";
      let result = remove_element("#simplabsheader", response_text);
      assert_eq!(result, "<!DOCTYPE html><html><head></head><body> Some content </body></html>");
    }
}

