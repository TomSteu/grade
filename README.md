Grade - A macro for rapid gtk-rs development
============================================

Grade is a macro for declarative development of [gtk-rs](https://gtk-rs.org/). It provides the macro `build!` that creates a GTK widget from declaration.

```rust

use gtk::*;
use grade::build;

let app = Application::new(
	Some("com.github.grade.test_app"), 
	Default::default()
).unwrap();

let main_window = build! {
	ApplicationWindow {	
		name: "app_window",
		title: "Grade test",
		application: &app,
		show_menubar: false,
		default_height: 300,
		default_width: 400,
		
		-- Viewport {
			-- main_grid: Grid {
			
			}
		}
	}
}
```