Grade - A macro for rapid gtk-rs development
============================================

Grade is a macro for declarative development of [gtk-rs](https://gtk-rs.org/). It provides the macro `build!` that creates a GTK widget from declaration.

```rust

use gtk::*;
use grade::build;

// build! macro provided by this crate
let main_window = build! {

	// any Object that has a corresponding ObjectBuilder
	Window {
		
		// properties (methods to the ObjectBuilder)
		title: "Grade test",
		show_menubar: false,
		default_height: 300,
		default_width: 400,
		
		// conncect signals of the Object
		=> activate_focus: |_| { /*...*/ },
		=> notify(title): |_| { /*...*/ }, 
		
		// children
		-- Viewport {
		
			// named object, for to reference it 
			-- main_grid: Grid {
				vexpand: true,
				name: "main_grid",
				
				// child added with `parent.attach(&child, 1,1,1,1);`
				// the default `--` is short for `-- [add]`
				-- [attach, 1, 1, 1, 1] Label {
					label: "Please type...",
				}
				
				// cascade-style syntax for methods of the parent
				..insert_row(2)
				
				// of course, `build!` is nestable 
				..attach(
					& build!(/*...*/).downcast::<Widget>(), 
					1, 2, 3, 3
				)
			}
		}
	}
};
```

