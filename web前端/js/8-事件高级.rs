1：注册事件
	给元素添加事件，称为注册事件或者绑定事件。
	
	有两种方式：
		传统方式： 都是以on开头的（onclick）
			特点：注册事件的唯一性，同一个元素同一个事件只能设置一个处理函数，最后注册的处理函数将会覆盖前面注册的处理函数。
		方法监听注册方式：
			eventTarget.addEventListener(type，listener，useCapture)它是一个方法。
			type: 事件类型字符串，比如click，mouseover这里不需要on
			listener：事件处理函数，事件处理发生时，会调用该监听函数
			useCapture：布尔值，默认false。true表示捕获阶段   false：表示冒泡阶段
			seed中的ev()函数就是这种效果的。
	
2：删除事件
	传统的方式是设置为none。
	方法监听注册方式的解绑：
		eventTarget.removeEventListener(‘click’, fn_name);
		在seed中，移除的方法是利用model里面的状态，在view中使用IF利用浏览器的重绘。
		
3: DOM事件流
	捕获阶段  Document -> Element HTML -> Element Body -> Element div 
	当前目标阶段
	冒泡阶段 然后再按照相反的顺利返回
	事件发生时会在元素节点之间按照特定的顺序传播，这个传播过程即DOM事件流。
	1： Js代码只能执行捕获或者冒泡其中的一个阶段。
	2：onclick和attachEvent只能得到冒泡阶段。
	我们一般使用到冒泡。seed中的ev使用的是冒泡的方法。
	
	得到事件的触发元素：
	event.target返回的是触发这个事件的元素.
	web_sys::Event::target(): 触发这个事件的元素
	web_sys::Event::current_target():绑定这个事件的元素
	
	事件的类型：
	e.type 
	web_sys::Event::type_();
	
	阻止默认行为：让链接不跳转，让提交不提交
		e.prevetDefault():阻止默认的行为。
		web_sys::Event有这个方法 prevent_default().
	
	阻止冒泡：
		冒泡的特性会带来一定的坏处，我们需要进行阻止。
		e.stopPropagation();
		web_sys::Event::stop_propagation();
		web_sys::Event::stop_immediate_propagation();
		
	事件委托：
		事件冒泡呢会给我们带来一些坏处，也会带来好处。
		事件委托的原理：不要给每一个子节点单独设置事件监听器，而是事件监听器设置在父节点上，然后利用冒泡原理影响设置每一个子节点。
		然后在父节点的监听程序中，利用event.target获取到响应的子节点。
		
常用的鼠标事件：
	1：禁止鼠标右键菜单
		contextmenu主要控制应该何时显示上下文菜单，主要用于程序员取消默认的上下文菜单。
		web_sys中，Ev::ContextMenu，
		ev(Ev::ContextMenu, |e| {e.prevent_default()})
	2：禁止鼠标选中
		selectstart：
	
event对象代表事件的状态，跟事件相关的一系列信息的集合。我们主要学习MouseEvent和KeyboardEvent事件对象。
	elemen.addListener("click", function(e) {});
	意思是说click这个事件发生了，就会产生一个事件对象，event对象。
	
	先来学习event事件对象中的鼠标对象：MouseEvent对象：
	e.clientX 	返回鼠标相对于浏览器窗口可视区的x坐标
	e.clientY 	返回鼠标相对于浏览器窗口可视区的y坐标
	e.pageX 	返回鼠标相对于文档页面的x坐标	常用
	e.pageY		返回鼠标相对于文档页面的y坐标	常用
	e.screenX 	返回鼠标相对于电脑屏幕的x坐标
	e.screenY	返回鼠标相对于电脑屏幕的Y坐标
	
常用的键盘事件：
	onkeyup：某个键盘按键被松开的时候触发 不区分大小写
	onkeydown：某个键盘按键按下时触发	不区分大小写
	onkeypress：某个键盘按键被按下时触发，但是它不识别功能键比如 ctrl shift 箭头等等。区分大小写
	顺序 down -> press -> up
	键盘事件对象：
		KeyboardEvent：
			key_code() -> u32;
			char_code() -> u32; 这是一个非标准的，尽量不要使用。
			code() -> String;
			key() -> String; 使用这个。
	
	
	
		
	
	
	
	