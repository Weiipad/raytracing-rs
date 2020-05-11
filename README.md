# Ray Tracing in Rust
-----------------------
这是我学习[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)时所使用的仓库
渲染出的图片都在achievements里。

## 我擅作主张，修改得和原书不一样的部分：

 + 由原教程的保存ppm改成了用[image crate](https://crates.io/crates/image)保存png。
 + tuple结构体好像用别名不太好用，就一直用Vector3了。
 + 原教程把向量的乘法重载为了逐分量相乘，这里我改成了叉积。逐分量相乘变成了一个成员函数。
 + 原教程的单线程渲染被我改成了多线程渲染（之前有一段时间实在是太慢了...）。
 + rand用的是rust的rand库，调C的rand()在多线程里出事了，大大减慢了速度（根本没跑完）。
 + 利用rust的tuple，所有的out型参数都被我改成了返回。