# RustyVault

bin/rusty_vault.rs
创建应用程序

## 阅读 Commit 记录

### 97b0ea7 Init Commit

添加两行的 README，协议，gitignore

### 368a9f6 Add requirement and design doc

design.md:

1. 虚线框中为RustyVault，其整体上分为三大部分：RustyVault Core, RustyVault Modules和RustyVault Surface.

RustyVault
|---Core
    |---Module Manager
    |---Storage Manager
|---Modules
    |---PKI Module
    |---Crypto Module
|---Surface
    |---HTTPS

- RustVault Core，此组件是RustyVault的核心组件，由多个Manager组成，不同的Manager分管不同功能。例如Module Manager负责对RustyVault的各个功能模块进行管理，提供模块的热插拔等机制；Crypto Manager则对接底层密码库（铜锁），通过rust-tongsuo的Rust binding来调用铜锁的C API实现各种密码学功能等；
- RustVault Modules，此部分由多个Module组成，是RustyVault的实际执行各种功能的环节，即RustyVault的实际功能代码都位于此。例如，PKI Module提供了完整的PKI能力，如扮演CA进行X.509证书的签发、吊销等管理工作；Crypto Module则依赖于RustyVault Core中的Crypto Manager对底层密码学原语进行调用，以实现对外提供诸如加密解密、签名验签等功能；
- RustyVault Surface，此组件是直接和最终用户打交道的部分，对外提供HTTPS接入能力，并对API请求进行解析后，转发给某个实际功能的Module上，由该Module进行处理后，返回处理结果给用户。此外，此组件还负责整体的配置解析等工作。

2. RustyVault需要依赖于底层的密码学算法库（也可称之为软件密码模块），由底层密码库提供全部的密码学相关功能。RustyVault默认的底层密码库是铜锁。

3. 密码硬件，如加密机、加密卡等，的使用对RustyVault是透明的，该过程由铜锁屏蔽，因此RustyVault对于硬件的差异和对接是无感的。

4. RustyVault中的敏感安全参数（如各种密钥、随机数、认证信息等）中存在持久化存储需求的，可以在本地加密存储，或者连接外部存储（如etcd）。连接外部存储对于创建RustyVault集群是必须的。存储方面的管理由RustyVault Core中的Storage Manager负责，RustyVault的其他组件也无需感知不同存储方式之间的使用差异。

req.md:

- 动机：HashiCorp Vault是云原生领域被广泛使用的Secret Management产品，也是CNCF的项目。在实际使用中，Vault存在一些不足，例如：……

- 需求列表：
  - API
  - 用户和认证
  - 传统密码学算法支持
  - ……

### 7be7e23 README

对README做了修改.

### b141a70 Update the architect design, #1

<https://github.com/Tongsuo-Project/RustyVault/issues/1>

- Surface更名为Interface
RustyVault
|---Interface
    |---HTTPS
|---Modules
    |---PKI Module
    |---Crypto Module
|---Core
    |---Module Manager
    |---Storage Manager
- Core和Module是Crate，这个Crate可以让外部创建叫做RustyVault的实例，其他Rust应用可以调用RustyValut
- CLI 和 RESTFul API应该调用相同的功能代码，而非CLI调用REST API的接口去工作，CLI可以提供REST API不合适提供的功能，譬如导出Storage的数据用于备份

# 6ee828e Merge pull request #2 from genedna/main
更新架构图，上一个commit是没有verfied，此次commit经过了verfied

# b8643b3 storage implementation.

Cargo.toml
src/
├── errors.rs
├── lib.rs
└── storage/
    ├── barrier.rs
    ├── barrier_aes_gcm.rs
    ├── barrier_view.rs
    ├── mod.rs
    └── physical/
        ├── file.rs
        └── mod.rs

- lib.rs 是library的入口点，这里声明了这个库有哪些模块
- 