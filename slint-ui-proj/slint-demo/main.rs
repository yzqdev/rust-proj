slint::include_modules!();
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    import { Button } from "std-widgets.slint";
export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
        Button {
            text: "click me";

    }
        }
    }
}
