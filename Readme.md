# Cygnus

Cygnus is a language that compiles to Apple Shortcuts.

I will create a guide when I have enough features.

## Plans for the future

The finished language should look something like the following, but I don’t think I’ll get that far.
Any changes to this sample code are welcome.

```
print => #{
	identifier: „is.workflow.showresult“
	parameters: {
		Text: string
	}
}

exit => #{
	identifier: // Currently unknown
	parameters: {}
}

print#("Hello World!");

say_hello(name: str) => !{
	return "print#(\"Hello \(name)\")";
}

say_hello!("Paul");

// A comment
/* Another comment
that stretches
over multiple lines */

/**
 * My function that does something
 */
fn my_function(arg: str) -> num {
	if arg == "hi" {
		print#("Hello!");
	} else if arg == "bye" {
		print#("See you later!");
	} else {
		print#("What did you say?");
	}

	for i in 5..10 {
		print#("Round \(i)");
	}

	var list: [num];
	// list.append(4); // Error: list isn’t initialized
	list = [];
	list.append(4);
	list.append(5);
	list.append(3);
	// `list` is currently [4, 5, 3]
	list.sort()
	// now `list` is [3, 4, 5]

	var dict = {"key": "value"} // infers type {str: str}

	each key, value in dict {
		each item in list {
			print#("\(key): \(value) => \(item)");
		}
	}

	if arg == "nothing" {
		return dict.len() + list.len();
	}

	return dict.len()
}

// print#("\(list)") // Error: Can’t find variable `list` in the current scope

print#("\(my_function('hi')");

type MyType = bool;

pub trait MyTrait {
	type Prop;

	fn get_prop(self) -> Prop;

	fn get_id(self) -> int {
		if self.get_prop() == "hi" {
			return 352;
		}

		return 532;
	}
}

struct MyStruct: MyTrait {
	pub my_property: str
	my_private_property: MyType

	pub fn new(my_property: str) -> Self {
		return Self {
			my_property, // If the identifier and the name of the assigned variable is
							 // the same, you can omit the colon and name of the identifier
			my_private_property: false
		};
	}

	pub fn get_private_prop(self) -> bool {
		self.print_private_prop();
		return self.my_private_property;
	}

	fn print_private_prop(self) {
		print#("private property is \(self.my_private_property)");
	}

	type Prop = MyType;

	fn get_prop(self) -> Prop {
		return self.	my_private_property;
	}
}

// You could do this to create a new instance:
var struct_instance = MyStruct {
	my_property: "hi", my_private_property: false
};

// But it is generally better to include a `new` method on the struct like this:
var new_struct_instance = MyStruct::new("hi");

var first_property = new_struct_instance.my_property;
var id = new_struct_instance.get_id();
/*
var private_property = new_struct_instance.my_private_property
// Error: Cannot access private members outside of struct
*/

/*
// Won’t work because of the limitations of the Shortcuts App
fn recursive(value: num, end: num) -> bool {
	if value == end {
		return true;
	} else {
		return recursive(value + 1, end);
	}
}
*/

enum Option<T> {
	case Some(T);
	case None;

	pub fn unwrap(self) -> T {
		match self {
			Some(value) => return value,
			None => {
				print#("Unwrapping failed! Value was none.");
				exit#();
			}
		}
	}
}

enum Direction {
	case Up;
	case Down;
	case Right;
	case Left;

	pub fn to_number(self) -> num {
		match self {
			case Up => return 1,
			case Down => return 0,
			case Right => {
				first_number = 1;
				return first_number + second_number;
			}
			_ => 2
		}
	}
}

var direction = Direction::Right;
print#("direction: \(direction)")

fn returns_trait(struct_with_trait: impls MyTrait) -> impls MyTrait {
	print#("\(struct_with_trait.get_id())");

	return struct_with_trait;
}
```

### Types

-   `num`
-   `str`
-   `bool`
-   `[type] // array`
-   `{type:type} // dictionary`
-   `date`
