触发 — 响应的机制

事件由三部分组成： 事件源   事件类型   事件处理程序 三要素
	1：事件源：事件被触发的对象 谁 按钮
		let byn: Option<Element> = this_document.get_element_by_id("btn");
	2：事件类型
		比如触发 什么事件 比如鼠标(onclick) 还是鼠标经过 还是键盘按下
	3：事件处理程序
		通过一个函数赋值的方式完成
		
set_onclick()函数只有HtmlElement有这个方法。
	
1：在rust中，我们推荐的是html元素和事件一起使用。但是有些情况，我们需要在update中，获取响应元素的事件，或者修改，获取获取。那么我们采取js的使用方式。
	let btn: Option<Element> = this_document.get_element_by_id("btn");
	btn.map(|e| {
		// btn是Element类型，我们需要将其转换为HtmlElement类型，才能设置事件，因为事件方法是HtmlElement的方法。
		let btn: HtmlElement = wasm_bindgen::JsValue::from(e)
		.unchecked_into::<web_sys::HtmlElement>();
		
		// 设置事件函数，set_onclick的参数是一个Function类型，来自js
		// new_no_arg()的参数是Function函数体内的字符串，这个字符串就是一个js的语句。
		btn.set_onclock(Some(Function::new_no_arg("alter(‘’hello)")));
	})
	
2：一般在rust的seed中，我们把事件处理和html的标签放在一起：
	input![
            C![C.h_4],
            attrs! { At::Value => model.value },
            ev(Ev::Click, |_event| { Msg::AddOne }),
        ]
这样，当上面的按钮按下的时候，我们不再需要像第一种方法那样写js的代码了，而是直接调用rust的代码了，比如这里的|_event| { Msg::AddOne }。注意这里的事件一般是变化为rust可以让update函数处理的消息。
seed中的ev的效果其实是类似于addEventListener的效果，同一个元素同一个事件也可以添加多个处理函数的。

执行事件的步骤：
1：获取事件源
2：注册事件（绑定事件）
3：添加事件处理程序




