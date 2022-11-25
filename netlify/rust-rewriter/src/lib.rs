use wasm_bindgen::prelude::*;
use chrono::prelude::*;
extern crate kuchiki;
use kuchiki::traits::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn should_remove_header(date_to: String, redirected: bool) -> bool {
  match Utc.datetime_from_str(&date_to, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => {
      let now = Utc::now();
      let is_in_future = date > now;
      is_in_future && redirected
    },
    Err(_)=> false,
  }
}

fn remove_element(id: &str, html: &str) -> String {
  let document = kuchiki::parse_html().one(html);

  match document.select_first(id) {
    Ok(element) => element.as_node().detach(),
    Err(()) => (),
  };

  return document.to_string();
}

#[wasm_bindgen]
pub fn remove_header(response_text: &str, date_to: &str, redirected: bool) -> String {
  if should_remove_header(date_to.to_string(), redirected) {
    return remove_element("simplabsheader", response_text);
  }
  return response_text.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greet("Stan");
        assert_eq!(result, "Hello, Stan!");
    }

    fn get_date(days: i64) -> String {
      let now = Utc::now();
      let date = now + chrono::Duration::days(days);
      date.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[test]
    fn it_returns_false_when_wrong_date(){
      let wrong_date = "1.1.asdfasdf";
      // call should_remove_header with wrong date
      let result = should_remove_header(wrong_date.to_string(), true);
      assert_eq!(result, false);
    }

    #[test]
    fn it_remove_header_when_redirected(){
      let date = get_date(2);

      let result = should_remove_header(date, true);
      assert_eq!(result, true);
    }

    #[test]
    fn it_doesnotremove_header_when_notredirected(){
      let date = get_date(2);

      let result = should_remove_header(date, false);
      assert_eq!(result, false);
    }

    #[test]
    fn it_doesnotremove_header_when_redirected_past(){
      let date = get_date(-2);

      let result = should_remove_header(date, true);
      assert_eq!(result, false);
    }

    #[test]
    fn it_doesnotremove_header_when_notredirected_past(){
      let date = get_date(-2);
      let result = should_remove_header(date, false);
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
}

