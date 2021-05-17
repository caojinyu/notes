BOM:浏览器对象模型，和浏览器窗口做交互动作。核心对象是window。
BOM是没有标准的，兼容性比较差。

BOM中的顶级对象是window。BOM学习的是浏览器窗口交互的一些动作。

window对象 = document + location + navigation + screen + history.

1: window是js访问浏览器窗口的接口。
2：全局变量，全局函数会自动变成window的属性和方法。只不过调用的时候可以省略。

window对象的常用事件：
	窗口加载事件：
	onload事件：当文档内容加载完毕之后出发该事件。

	定时器：
		setTimeout：
		seed中使用interval
        	orders.stream(streams::interval(1000, || Msg::Load));
        或者
			let handle =
        orders.stream_with_handle(streams::interval(1000, || Msg::Load));
		定时器的清除，必须使用handler的方法，当handler发生drop的时候，定时器就会被清除了。
		
	location对象：
		window对象给我们提供了一个location对象属性，用于获取或设置窗体的URL，并且可以用于解析URL。因为这个属性返回的是一个对象，所以
		我们称这个属性为location对象。
		location.href ：获取或设置整个URL
		localtion.host：返回主机（域名）www.baidu.com
		location.port: 返回端口号，如果没有写 返回空字符串
		location.pathname： 返回路径
		location.search: 返回参数
		location.hash： 返回片段 #后面内容 常见于链接锚点
	location中常见方法：
		location.assign() 跟href一样，可以跳转页面（也称为重定向页面）
		location.replace() 替换当前页面，因为不记录历史，所以不能后退页面。
		location.reload() 重新加载页面，相当于刷新按钮或者f5 如果参数为true强制刷新。
		
		
	navigator对象：
		navigator对象包含有关浏览器的信息，它有很多属性，我们最常用的是userAgent，
		该属性可以返回由客户端主机发送服务器的user-agent头部的值。前端代码可以判断用户哪个终端打开页面，实现跳转。
		
	history对象：
		与浏览器的历史记录进行交互，该对象包含用户（在浏览器窗口中）访问过的URL。
		history对象的方法：
			back()： 可以后退功能。
			forward(): 前进功能。
			go(参数): 前进后退功能，如果是1就是前进1个页面，如果是-1就是后退一个页面。
			
			
		
		