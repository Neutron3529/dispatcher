# 任务分发器

这是一个（大致）按顺序的多线程任务分发器

用法是

```bash
cat tasks.sh | cargo run --release 10
```

这可以以10线程将`tasks.sh`中的每一行分别以`sh -c`（或者windows系统下,`cmd /C`,如果你将cat改成windows下的type）的方式执行。

一个妙用是下载m3u8文件：

```bash
grep -v "#" index.m3u8 | sed "s/https/wget https/g" | cargo run --release 10
```

有爱自取，有bug的话，windows提PR，linux提issue。

windows的支持只是顺手写的，我更希望windows死，这样可以少维护不少东西。
