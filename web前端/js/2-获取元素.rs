1：根据ID获取
2：根据标签名获取
3：通过html5的新方法获取
4：特殊元素获取

在web-sys中：

pub type Document;
impl Document {
	pub fn document_element(this: &Document) -> Option<Element>;
	pub fn body() -> Option<HtmlElement>;
	pub fn get_element_by_id(element_id: &str) -> Option<Element>;
	pub fn get_elements_by_tag_name(local_name: &str) -> HtmlCollection;
	pub fn get_elements_by_class_name(class_names: &str) -> HtmlCollection;
	pub fn query_selector(this: &Element, selectors: &str) -> Result<Option<Element>, JsValue>;
	pub fn query_selector_all(this: &Element, selectors: &str) -> Result<NodeList, JsValue>;
} 

pub type Element;
impl Element {
	pub fn get_element_by_id(element_id: &str) -> Option<Element>;
	pub fn get_elements_by_tag_name(local_name: &str) -> HtmlCollection;
	pub fn get_elements_by_class_name(class_names: &str) -> HtmlCollection;
	pub fn query_selector(this: &Element, selectors: &str) -> Result<Option<Element>, JsValue>;
	pub fn query_selector_all(this: &Element, selectors: &str) -> Result<NodeList, JsValue>;
}

let this_document = document(); // Document

1：getElementById(&str) -> Option<Element> 方法获取带有id的元素对象。
	在web-sys中：
		let this_document = document();
		let id_app: Option<Element> = this_document.get_element_by_id("app");
		
2：getElementByTagName(&str) -> Option<Vec<Element>> 返回带有指定标签名的对象的集合。
	web-sys:
		let lis: HtmlCollection = this_document.get_elements_by_tag_name("li");
		let lis0: Option<Element> =lis.get_with_index(0);
	也可以获取元素中的元素：
		let ols: HtmlCollection = this_document.get_elements_by_tag_name("ol");
		let lis: HtmlCollection = ols.get_with_index(0).unwrap().get_elements_by_tag_name("li");

3：html3新增的方法，通过类型获取：
	fn get_elements_by_class_name(class_names: &str) -> HtmlCollection;

	返回指定选择器的第一个元素对象：
	fn query_selector(this: &Element, selectors: &str) -> Result<Option<Element>, JsValue>;
	let box: Result<Option<Element>, JsValue> = query_selector(".box"); // 如果有好多使用box的类，但是返回的是第一个
	let nav: Result<Option<Element>, JsValue> = query_selector("#nav"); // 返回的nav元素的第一个。

	// 获取指定选择器的所有NodeList。
	pub fn query_selector_all(selectors: &str) -> Result<NodeList, JsValue>;
	let boxs: Result<NodeList, JsValue> = query_selector_all(".box");
	
4：特殊标签的获取 body  html
	获取body标签：
		let body: Option<HtmlElement> = this_document.body();
	
	获取html标签：
	 	let html:Option<Element> = this_document.docuemnt_element();
	
	
	
	