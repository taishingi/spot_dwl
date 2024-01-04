pub mod spot {
    use std::process::Command;

    pub struct Notification {
        icon: String,
        summary: String,
        body: String,
        app: String,
        timeout: i32,
    }

    impl Default for Notification {
        fn default() -> Self {
            Self {
                icon: String::new(),
                summary: String::new(),
                body: String::new(),
                app: String::new(),
                timeout: 5000,
            }
        }
    }

    impl Notification {
        #[must_use]
        pub fn new() -> Self {
            Self {
                icon: String::new(),
                summary: String::new(),
                body: String::new(),
                app: String::new(),
                timeout: 5000,
            }
        }

        pub fn summary(&mut self, text: &str) -> &mut Self {
            self.summary.clear();
            self.summary = text.to_string();
            self
        }

        pub fn timeout(&mut self, time: i32) -> &mut Self {
            self.timeout = time;
            self
        }

        pub fn body(&mut self, text: &str) -> &mut Self {
            self.body.clear();
            self.body = text.to_string();
            self
        }

        pub fn app(&mut self, name: &str) -> &mut Self {
            self.app.clear();
            self.app = name.to_string();
            self
        }

        pub fn icon(&mut self, name: &str) -> &mut Self {
            self.icon.clear();
            self.icon = name.to_string();
            self
        }

        /// # Panics
        ///
        /// Will panic if notify-send is not founded
        ///
        pub fn send(&mut self) -> bool {
            if self.app.is_empty() {
                Command::new("notify-send")
                    .arg("-t")
                    .arg(self.timeout.to_string().as_str())
                    .arg("-i")
                    .arg(self.icon.as_str())
                    .arg(self.summary.as_str())
                    .arg(self.body.as_str())
                    .spawn()
                    .expect("Fail to launch notify-send")
                    .wait()
                    .expect("msg")
                    .success()
            } else {
                Command::new("notify-send")
                    .arg("-t")
                    .arg(self.timeout.to_string().as_str())
                    .arg("-a")
                    .arg(self.app.as_str())
                    .arg("-i")
                    .arg(self.icon.as_str())
                    .arg(self.summary.as_str())
                    .arg(self.body.as_str())
                    .spawn()
                    .expect("Fail to launch notify-send")
                    .wait()
                    .expect("msg")
                    .success()
            }
        }
    }
}
