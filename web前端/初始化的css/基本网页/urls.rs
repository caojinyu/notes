use seed::prelude::*;
use seed::struct_urls;

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn html1(self) -> Url {
        self.base_url().add_path_part("html1")
    }
}
