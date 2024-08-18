# 🏗️ 开发环境搭建

<figure><img src="https://code.visualstudio.com/assets/home/home-screenshot-mac-2x-v2-light.png" alt=""><figcaption><p>VS code 代码编辑器</p></figcaption></figure>

## 搭建开发环境

{% hint style="info" %}
推荐使用[在线开发环境](quickstart.md#zai-xian-kai-fa-huan-jing-gou-jian)
{% endhint %}

### 本地环境搭建

1. 安装rust 开发环境 (1. windows 用户点击这里[安装包下载](https://www.rust-lang.org/zh-CN/tools/install) 2. Liunx or MacOS onclick there [代理下载](https://rsproxy.cn/) )
2. 安装喜欢的开发编辑器这里会给出常用的代码编辑器 [VSCODE](https://code.visualstudio.com/) （点击到达下载页面）、RustRover、Vim、Zed （这里只演示 VSCODE）
3.  安装工具（换源解决下载网络速度（Linux and MacOS 在第一步已经完成， windows用户请注意这里）、代码提示解决记不住单词关键字（每个人都要做的3.2））

    1.  切换国内源 [https://github.com/wtklbm/crm](https://github.com/wtklbm/crm)

        ```sh
        cargo install crm
        ```
    2.  使用crm切换&#x20;

        ```shell
        crm use rsproxy
        ```


4.  添加VSCODE插件（RustRover 用户开箱即用在外面学习阶段不需要添加其他插件，当然AI助手你想添加可自行添加 ）

    [https://code.visualstudio.com/docs/languages/rust](https://code.visualstudio.com/docs/languages/rust)

<div data-full-width="false">

<figure><img src="https://code.visualstudio.com/assets/docs/languages/rust/rust-analyzer-extension.png" alt=""><figcaption><p>rust-analyzer(代码分析工具)</p></figcaption></figure>

</div>

### 在线开发环境构建

1. 在github上创建仓库，进入[codespaces](https://github.com/Outlier-2/learn-to-develop-with-the-rust/tree/Rust-the-development-environment) （也可点击这里的仓库直接进入环境的分支进行开发）
2. 安装插件(同上)
