# Yet Another Static Site Generator

Very simple static site generator that compiles files to plain HTML

Features include:
- Custom components
- Compiling to plain HTML
- Multiple pages
- Watching for file changes

## Installation

First, you have to install [Rust](https://rustup.rs/). Then, install Yassg by executing:

    $ cargo install yassg

If you want to install the unreleased version from Git, clone this repo, and then run `cargo install --path .`

## Getting started

### Project structure
Create a folder for your project. In the new `src` directory, create a file `index.html` and a folder named `components`.  
The file structure should look like this:
```
my-project
└── src
   ├── components
   └── index.html
```

### Components and variables
Inside the `components` folder, create a file `Greeting.html`, or whatever name you'd like. It must start with an uppercase letter and not contain a space. PascalCase is recommended.

Inside this file put some html, for example
```html
<div>
    <p>Hello, {{name}}!</p>
</div>
```

`{{name}}` is a variable referenced in the HTML. We assign them values when using the components.

Open `index.html`, and insert this code:
```html
<!DOCTYPE html>
<html>
<head></head>
<body>
    <Greeting name="World" />
</body>
</html>
```

Remember the name we gave to the file in the components directory? We use the same name (without the file extension) in our HTML to use the component. We then assign value `World` to the variable `name` we used in our component.

### Generating the site
After we added some code, we can run Yassg to compile it. 

    $ yassg .

> **Info**  
> If you want Yassg to automatically rebuild the project when you change something, pass in the `--watch` flag
	
It will read all component files, and place them inside the `index.html` file, with the assigned variables, resulting in the following output in the `dist` directory:
```html
<!DOCTYPE html>
<html>
<head></head>
<body>
    <div>
        <p>Hello, World!</p>
    </div>
</body>
</html>
```

Congrats on making your first static page with Yassg!

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/rafal06/yassg.

## License

Yassg is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
