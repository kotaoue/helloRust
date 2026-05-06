slint::slint! {
    export component MainWindow inherits Window {
        title: "Hello World for Slint";
        width: 320px;
        height: 240px;

        Rectangle {
            background: #ff00ff;

            VerticalLayout {
                alignment: center;

                Text {
                    text: "Hello, World!";
                    color: white;
                    font-size: 28px;
                    horizontal-alignment: center;
                }

                Text {
                    text: "This is a simple Slint application.";
                    color: white;
                    horizontal-alignment: center;
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;
    app.run()
}
