# Lambda Calculator
This is a very simple Rust program as of now, but I'm planning on adding functionality for macros to make it easier to write lambda calculus programs. 
# How Does it Work?
Similar to [my inspiration](https://lambdacalc.dev/) to write lambdas you use the **'\\'** character, and the rest is just lambda calculus. I will
most likely add more information on how to write lambda calculus programs as I learn more about it.
# Macros
Writing raw lambda calculus programs can be very complicated, especially when working with several control flow statements. I've introduced some syntactic sugar by being able to define macros within the program. The way they work is by typing: `#macro_name = \x.x`, where the `\x.x` can be replaced by any valid lambda calculus statement. To place these within your program you can then use `#macro_name`. You can even use already defined macros to define new macros ej. `#macro_name2 = \x.#macro_name x`, which can make it easier to code more complicated programs.
# Roadmap
Although these aren't garuntees, I want to make the parser its own seperate project so I can use it to make other programming languages. In terms of the project, it is basically already complete, so the only updates to it would be if any bugs are found in the code, if I want to update the README with more information on lambda calculus to make it more approachable, or if I want to make my code more readable.