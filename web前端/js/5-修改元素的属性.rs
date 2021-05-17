比如我们通过按钮修改img<src = "">, 修改src的属性，
获取到img标签会后，img_name.src = "…"; 直接赋值就可以了。

在seed中，set_src()函数可以达到效果，但是要注意类型。
let img_name = this_document.get_element_by_id("img_id");
img_name.map(|e| {
	// 这里的e是一个Element类型。并没有set_src这个方法，我需要强制转换
	let e: HtmlImageElement = wasm_bindgen::JsValue::from(e)
	.unchecked_into();
	e.set_src = "…";
});

但是在seed中，我们并不会按照js的方法去做，为什么呢，因为seed中的属性，我们可以通过rust代码去实现。
vec![
        button!(
            "刘德华",
            ev(Ev::Click, |_event| {
                Msg::ChangedImg("../../static/images/pig.jpg")
            })
        ),
        button!(
            "张学友",
            ev(Ev::Click, |_event| {
                Msg::ChangedImg("../../static/images/1.jpeg")
            })
        ),
        img!(attrs!(At::Src => model.img_path)),
    ]
  这里的Src是一个灵活的变量。
  
  同理在seed中C![value_str], 类也是一个变量了，不需要用js暴力的去操作了。
  
  表单元素的属性操作：
  	type、value、check、selected、disabled
  	上面的这些不能再使用set_inner_html修改内容了。
  	
 样式属性操作：
  js中element.style = "", element.className = ""
  web_sys中，set_class_name(&str), style没有找到。
  seed中，C![] 和
  style!{
  	St::Margin => px(50),
        St::MaxWidth => unit!(50, %),
        St::Top => 0,
        St::Padding => px(20) + " " + &px(15)
        St::BackgroundColor => if selected { "green" } else { "white" },
        St::from("custom_name") => IF!(apply_custom => "a_value"),
  }
 更一般的形式：
 attrs!{At::Class => "class_a", At::Style => "top:0"}
 
 attrs!属性宏：
属性名称： 如：At::TiTle
值：可以是AtValue或者是实现ToString的所有值。值有三种方式：
	Ignored： 忽略整个属性
	None：不使用属性值。空字符串
	Some(String)：如果v在中At::X => v，ToString则将其自动转换为AtValue::Some(v)。
  as_at_value：
let disabled = false;
...
attrs!{
    At::Disabled => disabled.as_at_value()
}替代：
attrs!{
    At::Disabled => if disabled { AtValue::None } else { AtValue::Ignored }
}

js的class修改使用seed的C![],
js的style修改使用seed的style!{ St::Background => "" }代替.


获取属性的值：
    element.属性：获取内置的属性
    element.属性= ‘’; 设置内置的属性值
    
    element.getAttribute(‘’)：获取我们程序员自定义的属性。
    element.setAttribute(‘’, value)：设置程序员自定的属性值。但是class比较特殊，可以使用这个设置class。
    element.removeAttribute("")：移除自定义的属性。
    在seed中，自定义的属性，可以通过attrs!{ "index" => 10 }, 这样去定义。H5为了解决让程序员区分是内置属性还是自定义属性，添加了，设置自定义属性时，必须是以data-开头，  比如data-index = 1;
    这样我们在获取自定义属性的时候，可以使用element.dataset.index;dataset里面存放了所以以data开头的自定义属性。或者element.dataset["index"]， 要是有一个自定义变量data-list-name; 那么使用element.dataset.listName, 使用驼峰命名法替代‘-’.
    seed中：
        let this_document = document();
            let input_e: Option<Element> =
                this_document.get_element_by_id("input_1");
            input_e.map(|e| {
                e.get_attribute("data-list-name").map(|value| {
                    log!("data-list-name is ", value);
                })
            });
        
        attrs!(
                At::Type => "password",
                "data-list-name" => "hades",  // 自定义属性
                At::Id => "input_1"
            ),

    



