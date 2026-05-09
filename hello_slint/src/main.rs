slint::slint! {
    export component MainWindow inherits Window {
        title: "Hello World for Slint";
        width: 320px;
        height: 240px;

        Rectangle {
            background: #0033ff;

            VerticalLayout {
                alignment: center;

                Text {
                    text: "Hello, World!";
                    color: #ff0000;
                    font-size: 28px;
                    horizontal-alignment: center;
                }

                Text {
                    text: "This is a simple Slint application.";
                    color: #00ff00;
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
