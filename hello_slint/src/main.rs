slint::slint! {
    export component MainWindow inherits Window {
        title: "Hello World for Slint";
        width: 320px;
        height: 240px;

        Rectangle {
            background: @linear-gradient(0deg, #336699 0%, #4477aa 18%, rgb(126, 166, 207) 55%, #ccccff 100%);

            VerticalLayout {
                alignment: center;

                Text {
                    text: "Hello, World!";
                    color: #333333;
                    font-size: 28px;
                    font-family: "Helvetica";
                    horizontal-alignment: center;
                }

                Text {
                    text: "This is a simple Slint application.";
                    color: #666666;
                    font-size: 14px;
                    font-family: "Times New Roman";
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
