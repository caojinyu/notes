为什么需要node节点操作呢？
	因为以前获取元素的方法比较繁琐，逻辑性不强。
所以出现了利用节点层次关系进行操作。利用父子兄的关系。

一般的，节点至少拥有nodeType（节点类型）， nodeName（节点名称）， nodeValue（节点值）这三个基本属性。

element.parentNode  父节点
seed中： parent_noe(&Node) -> Option<Node>


childNodes  节点（包含文本节点等等我们不需要关心的）
children	元素
seed: child_nodes() children()

firstNode
firstElementChild

lastNode
lastElementChild
实际开发的方法：
	element.children[0]

兄弟节点：
	nextSibling: 下一个兄弟节点。
	nextElementSibling: 下一个元素节点。
	previousSibling: 上一个兄弟节点
	previousElementSibling: 上一个兄弟元素节点
	
创建节点：
	crateElement
		createElement比innerHtml的执行效率要好的多。
		但是用字符数组，先拼接好，然后再一次性渲染的效率是最高的。
	appendChild
	insertBefore

删除节点：
	removeChild
	
复制节点：
	cloneNode
	


