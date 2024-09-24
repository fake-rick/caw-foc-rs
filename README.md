# CawFOC (Rust)

---

本项目为CawFOC的新版固件，采用Rust语言开发，使用Embassy进行异步设计

```rust
# 可以通过以下代码配置6xPWM
let ch1n = ComplementaryPwmPin::new_ch1(r.tim1_ch1n, OutputType::PushPull);
let ch2n = ComplementaryPwmPin::new_ch2(r.tim1_ch2n, OutputType::PushPull);
let ch3n = ComplementaryPwmPin::new_ch3(r.tim1_ch3n, OutputType::PushPull);
```

## 注意

* DAP-LINK需要连接CawDrive的Reset引脚，否则probe-rs调试时会报错
* 烧录后需要拔掉DAP-LINK，或者将Reset线拔掉，否则STM32无法正常工作
