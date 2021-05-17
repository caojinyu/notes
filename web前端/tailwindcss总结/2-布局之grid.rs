grid布局：
	栅格的整个对齐，是因为整体的栅格没有完全占据盒子的情况下使用，
		justify-content: start | end | center | round | between | evenly;
		align-content: …
		记住用的是content表示整体的布局栅格布局。改变的是栅格的大小和位置。

		
	栅格的内部元素items的全部对齐方式：
		justify-items:
		align-items:
		记住用的是items表示的栅格内部元素的全部对齐方式。如果元素没有给固定的大小的话，会自动的拉扯的。如果给了固定的大小，元素大小不会改变。
		注意： 这里改变的是全部元素的对齐方式，栅格的大小和位置不会改变的。
		
	
	栅格内的元素独立控制对齐：
		justify-self:
		align-self:
		这里改变的是栅格内部元素的对齐方式，不会改变栅格的大小和位置。
	
	栅格对齐方式的简写：
		place-content: 垂直 水平；
		place-items: 垂直 水平;
		place-self：垂直 水平；