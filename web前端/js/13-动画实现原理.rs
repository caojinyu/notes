核心原理：
	通过定时器setInterval不断移动盒子位置。

实现步骤：
	1：获得盒子当前的位置。盒子要绝对定位。
	2：让盒子在当前位置上加1个移动距离。
	3：让定时器不断的重复这个动作。
	4：结束定时器的条件。
	
让动画慢慢的停下来：算法：
	（目标位置 - 当前位置） / 10; 当前的位置不停的在改变
	步长值需要取整的。