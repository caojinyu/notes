mouseenter事件：
	当鼠标移动到元素上就会触发mouseenter事件。
	类似mouseover事件，他们之间的差别是：
		mouseover鼠标经过自身盒子会触发，经过子盒子还会触发，
		mouseenter只会经过自身盒子触发。因为mouseover会冒泡，mouseenter不会冒泡。
		mouseenter对应的是mouseleave，也不会冒泡。
	