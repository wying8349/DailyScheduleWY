# DailyScheduleWY

## Day 1 2020/7/1
第一周先学习语言，阅读[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)
- 安装环境，安装ide（使用了Clion）
- Hello World! Hello Cargo! [猜猜看游戏](https://github.com/wying8349/DailyScheduleWY/tree/master/practice/tutorial/guessing_game)

## Day 2 2020/7/2
在忙其他的课业，预计7月9日才能正式结束

## Day 3 2020/7/3
陈老师针对我复习OS课程的情况布置了一些作业，今天复习了以下课程视频
- 操作系统概述
- 操作系统与系统结构和程序设计语言
- 中断、异常、系统调用
- [lec1的QA](https://shimo.im/docs/xDdvOYBJ0HMcbklk)

## Day 4 2020/7/4
生病去医院了

## Day 5 2020/7/5
继续病假。。

## Day 6 2020/7/6
忙于其他的课业。。
阅读[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)
- 所有权
- 结构体
- 枚举

## Day 7 
## Day 8 
## Day 9 
## Day 10 
## Day 11 
## Day 12 
## Day 13 
## Day 14 
## Day 15 
## Day 16 
## Day 17 
  
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
# 八月 
## Day 32
感觉对实验的掌握很不好，准备重新做一下试验，尽量自己能写能答题，今天检查了一下环境
[riscv手册](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf)
## Day 33
陈老师没空，和陈同学交流，了解到相关的项目已经开始了= =
重做lab0，lab1
## Day 34
重做lab2，和陈同学交流，旁听了一生一芯项目同学的交流
## Day 35
陈老师同意把我塞进去，感觉我真是太菜了
旁听了Sipeed工程师的交流
重做lab2
## Day 36
看了一下[仓库](https://github.com/rcore-os/zcore_tutorial_developers)里的相关资料，看得很乱，而且感觉不知道去哪里得到帮助……
今天会很多，上午参与了文档组的交流，可能不会写文档了，按照模块分配写一些unit test，我觉得也很好
晚上和陈同学聊了下lab3，顺便咨询了一下选择什么模块，陈同学建议选VMO,我对syscall有些偏好，我想先看点代码
## Day 35
找到VM相关的代码阅读，[VMO](https://rcore-os.github.io/zCore/zircon_object/vm/index.html)
 
今天要交流计划，计划如下：
根据仓库里的分工，文档分为四个方面，我复制一下

Fuchsia OS 和 Zircon 微内核

内核对象 1.1. 初识内核对象
 1.2. 对象管理器：Process 对象 <zircon-object\src\task\process.rs> job/process/thread
 1.3. 对象传送器：Channel 对象 <zircon-object\src\ipc\channel.rs>

任务管理
 2.1. Zircon 任务管理体系 <zircon-object\src\task>
 2.2. 硬件抽象层与异步运行时 <kernel_hal(bare)> async 《zCore 操作系统内核的设计与实现》中有相关描述
 2.3. 线程管理：Thread 对象 <zircon-object\src\task\thread.rs>std::thread(8.4日)
 2.4. 进程管理：Process 与 Job 对象 <zircon-object\src\task\job.rs> <zircon-object\src\task\job_policy.rs>

内存管理
 3.1. Zircon 内存管理模型
 3.2. 物理内存：VMO 对象 <zircon-object\src\vm\vmo\physical.rs>
 3.3. 虚拟内存：VMAR 对象 <zircon-object\src\vm\vmar.rs>

用户程序
 4.1. Zircon 用户程序
 4.2. 加载 ELF 文件 <zircon-object\src\util\elf_loader.rs>
 4.3. 上下文切换
 4.4. 系统调用 <zircon-syscall\src>

根据其他同学的分工，虽然大家说可能会重新规划，我可能会被安排写内核对象
Zircon is an object-based kernel. User mode code almost exclusively interacts with OS resources via object handles. A handle can be thought of as an active session with a specific OS subsystem scoped to a particular resource.

Zircon actively manages the following resources:

processor time
memory and address spaces
device-io memory
interrupts
signaling and waiting

Channel
A channel is a bidirectional transport of messages consisting of some amount of byte data and some number of handles.

A zircon process is an instance of a program in the traditional sense: a set of instructions which will be executed by one or more threads, along with a collection of resources.

Process
The process object is a container of the following resources:

Handles
Virtual Memory Address Regions
Threads
In general, it is associated with code which it is executing until it is forcefully terminated or the program exits.

Processes are owned by jobs and allow an application that is composed by more than one process to be treated as a single entity, from the perspective of resource and permission limits, as well as lifetime control.

 
### 目前对下一个月的计划如下
目标(可量化):
1. 和同学合作对zircon object进行unit test，主要包括对象管理器和对象传送器
2. 详细注释的代码，包括[task](https://github.com/rcore-os/zCore/tree/master/zircon-object/src/task)中的job.rs, process.rs, thread.rs, [ipc](https://github.com/rcore-os/zCore/tree/master/zircon-object/src/ipc)中的channel.rs和fifo.rs
3. 对代码的分析报告
 
#### Week1：
对代码进行详细注释，包括[task](https://github.com/rcore-os/zCore/tree/master/zircon-object/src/task)中的job.rs, process.rs, thread.rs, [ipc](https://github.com/rcore-os/zCore/tree/master/zircon-object/src/ipc)中的channel.rs和fifo.rs

#### Week2：
完善task部分的unit test(Process, thread, Job)
问题：对于zircon object如何进行unit test？尚不清楚，需要学习

#### Week3：
出一个task部分的分析报告，作为tutorial的雏形

难点：文档是一个整体，同学们之间究竟怎么组织规划？

