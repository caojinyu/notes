三大系列都是HtmlElement对象的方法。

impl HtmlElement {
	fn offset_top(&self) -> i32;
	fn offset_left(&self) -> i32;
	fn offset_width(&self) -> i32;
	fn offset_height(&self) -> i32;
	fn offset_parent(&self) -> Option<Element>;
}

offset系列：元素偏移量
	利用offset系列相关属性可以动态的得到该元素的位置（偏移）、大小等。
	获得元素距离带有定位父元素的位置，父亲没有定位以body为准。
	获得元素自身的大小（宽度高度）。
	注意返回的数值都是不带有单位的。
	fn offset_top(&self) -> i32;
		返回元素相对带有定位父元素上方的偏移。常用
	fn offset_left(&self) -> i32;
		返回元素相对带有定位父元素左侧方的偏移。常用
	fn offset_width(&self) -> i32;
		获取盒子的宽度（包括padding  border）
	fn offset_height(&self) -> i32;
		获取盒子的高度（包括padding  border）
	fn offset_parent(&self) -> Option<Element>;
		返回带有定位的父亲元素。如果父级没有定位，返回的就是body。
	offset和style的区别：
		style只能得到行内样式表的样式值并且带有单位。不包括padding和border的。是可读可写的属性。通过修改得到一些效果。
		offset都能得到，不带有单位。是只读属性，不能更改的。
		
元素可视区client系列：
	翻译过来就是客户端的意思，我们使用client系列的相关属性来获取元素可视区的相关信息。通过client系列的相关属性可以动态的得到该元素的边框大小、元素大小等。
	element.clientTop: 返回元素上边框的大小
	element.clientLeft: 返回元素左边框的大小
	element.clientWidth: 返回自身包括padding、内容区的宽度，不包括边框，返回数值不带有单位， 常用
	element.clientHeight: 返回自身包括padding、内容区的高度，不包括边框，返回数值不带有单位。常用
	
pageShow事件：重新加载事件，防止后退等操作有缓存的情况。

元素的scroll系列：
	element.scrollTop: 返回被卷去的高度距离盒子的top的边框下沿的距离，返回数值不带有单位
	element.scrollLeft：返回被卷去的左侧距离，
	element.scrollWidth：返回自身实际的宽度，不含边框
	element.scrollHeight：返回自身事件的高度，不含边框
	和client的本质区别：scroll是内容比较多的时候返回的是真正的内容的大小
	client返回的是盒子的大小。
	
window.pageXOffset()  window.pageYOffset() 表示html卷去了多少元素。

	
