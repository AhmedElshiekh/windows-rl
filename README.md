# 🛡️ Windows-RL: The Rust-Wine Manager
###  محرك لإدارة بيئات ويندوز بلغة راست

---

## 🌍 Overview | نظرة عامة

**English:** **Windows-RL** is a sophisticated graphical system manager built with **Rust**. It is designed to simplify the management and execution of Windows applications on **Linux** distributions. While it is built for desktop-class performance, it features exceptional support for mobile Linux environments (PRoot/Termux), making it a truly cross-platform tool for Wine/Box64 management.

**العربية:** **Windows-RL** هو مدير نظام رسومي متطور مبني بلغة **Rust**، صُمم لتبسيط إدارة وتشغيل تطبيقات ويندوز على توزيعات **Linux**. بينما تم تصميمه لتقديم أداء يضاهي الأنظمة المكتبية، فإنه يتميز بدعم استثنائي لبيئات لينكس المحمولة (PRoot/Termux)، مما يجعله أداة عابرة للمنصات لإدارة Wine و Box64.

---

## ✨ Key Features | المميزات الرئيسية

* **🚀 Agnostic Backend:** Native support for both **X11** and **Wayland** with smart session management.
    * *دعم شامل لبروتوكولات X11 و Wayland مع إدارة ذكية للجلسات.*
* **🏗️ Architecture Agnostic:** Optimized for both x86_64 (Desktop) and ARM64 (Mobile/SBC).
    * *متوافق مع معالجات x86_64 للحواسيب و ARM64 للأجهزة المحمولة.*
* **🦀 Rust-Powered Stability:** Minimal resource footprint with high-speed UI response using egui.
    * *استقرار فائق بفضل لغة Rust واستهلاك ضئيل للموارد مع واجهة egui السريعة.*
* **🛠️ Self-Healing Environment:** Automatically detects and installs missing dependencies.
    * *بيئة ذاتية الإصلاح تقوم بفحص وتثبيت الاعتماديات الناقصة تلقائياً.*
* **🛑 Task Control:** Integrated system to monitor and terminate hanging Wine processes safely.
    * *تحكم كامل بالمهام لمراقبة وإنهاء عمليات Wine العالقة بأمان.*

---

## 🏗️ Build & Run | البناء والتشغيل
 
**1. Clone the repository | استنساخ المستودع:**
```bash
cd windows-rl
```

**2. Build the project | بناء المشروع:**
```bash
cargo build --release
```

**3. Execution | التشغيل:**

For Desktop Linux
```bash
./target/release/windows-rl
```
For Android (Within PRoot):
```bash
export DISPLAY=:1
./target/release/windows-rl

```
---

## 📂 Project Structure | هيكلية المشروع

* **src/main.rs**: Central logic for session management and environment scrubbing.
    * *المنطق المركزي لإدارة الجلسات وتنظيف بيئة النظام.*
* **src/core/**: Package management engine and filesystem operations.
    * *محرك إدارة الحزم والعمليات الأساسية لنظام الملفات.*
* **src/engine/**: Binary execution logic and application lifecycle monitoring.
    * *منطق تشغيل البرمجيات ومراقبة دورة حياة التطبيقات.*

---

## 🛡️ Stability Philosophy | فلسفة الاستقرار

**English:** The project tackles common compatibility issues by implementing Environment Scrubbing (clearing conflicting variables) and SHM Management (disabling shared memory in isolated containers to prevent visual artifacts). It forces stable X11 rendering when Wayland protocols are incomplete in the host environment.

**العربية:** يعالج المشروع مشاكل التوافق عبر تقنيات تنظيف البيئة (مسح المتغيرات المتعارضة) و إدارة الذاكرة المشتركة SHM (تعطيلها في الحاويات المعزولة لمنع تشوه الرسوميات). كما يفرض النظام بروتوكول X11 المستقر في حال كانت بروتوكولات Wayland غير مكتملة في البيئة المضيفة.

---

## 🤝 Contributing | المساهمة

**English:** Contributions are welcome! Feel free to open a Pull Request to improve Wayland support or Wine protocol handling.  

**العربية:** المساهمات مفتوحة للجميع! لا تتردد في تقديم Pull Request لتحسين دعم وايلاند أو تطوير التعامل مع بروتوكول Wine.

---
**License:** MIT  
**Developed with ❤️ using Rust**
