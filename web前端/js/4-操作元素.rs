我们可以利用DOM操作元素里面的内容、属性等。

1：改变元素的内容：
	element.innerText: 从起始位置到终止位置的内容，但是它会除去Html标签，同时空格和换行也会去掉。这个设置的是值，不能解析各种标签。这个是非标准的。
	
	element.innerHTML: 起始位置到终止位置的内容，包括Html标签，同时保留空格和换行。这个可以正确的解析出各种标签。这个是w3c标准的。
	
注意在seed中，通过消息，然后改变model成员的值，view显示mode成员的值，这个是基于值的形式，也就是说基于set_inner_text()的形式，不能解析html的各种标签。

当然在seed中我们也可以这样做，通过在model设置一些状态，通过msg通知，调用update函数，修改model的状态，在view中判断model的状态，使用：
	div![IF!(model.yes => raw!("<p>这是生成的</p>"))],
这样就使用了set_inner_html()函数的效果了。


set_inner_text()
set_inner_html()
是js比较狂野的函数，可以在seed中不受束缚的使用。但是seed有自己的框架，可以按照seed的框架去处理。seed中set_inner_html可以使用raw!()宏替代。

document.write 会导致页面重绘，导致原来的页面没有了。
window.onload 也是这样的效果.