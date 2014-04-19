# Developing for Platypie  
  
So you're interested in contributing to the Platypie codebase.. We think that's wonderful! Discussion of development happens on [IRC](http://kernelmeltdown.org/blog/how-to-set-up-irc-using-hexchat-beginners-walkthrough/). Jump on [Freenode](http://webchat.freenode.net/?channels=%23platypie "freenode's webchat") in #platypie  
  
## How to format your code  
We do our best to ensure that those who want to help are made welcome. In the interest of keeping everybody happy, we want to produce easily maintainable code. That means keeping everything readable, and using algorithms whose nature and purpose is transparent to people other than technological wizards.  
  
Read [Mozilla's guide to Rust style](https://github.com/mozilla/rust/wiki/Note-style-guide) before starting. If your code isn't up to this standard, it won't be accepted. With that being said, we are happy to help you improve as a [FOSS](http://en.wikipedia.org/wiki/Free_and_open-source_software) Rust developer. There's no need to feel bad if your revision isn't accepted the first time around, it's a young language and we're fairly new to it ourselves.  
  
### Otherwise  
* Set your text editor or IDE to insert spaces instead of tabs. Indentation should be two (2) characters wide, and the terminating brace (where applicable) must be inserted at the same depth it began on.  
* Begin each source file with a multiline comment including:  
  1. the name of the file.
  1. a brief summary of the code's purpose.
  1. a list of the file's dependencies.
  
The reason for beginning with a multiline comment is that it will trick most text editors without Rust highlighting into treating the file as C code. We don't expect anyone to use one particular plugin or IDE, and this is an easy way to make the source a little more readable without requiring additional effort.  
  
## Additional concerns  
  
Code formatting is fairly simple to fix. Codebase issues are significantly more complicated. If your code requires additional dependencies, we hope you'll discuss it with us before jumping in. We don't want you to spend your time developing a solution only to have it turned down because of portability issues. If it comes down to it, though, portability **will** supercede your emotions. Stop by the official IRC channel and ask someone who has been *voiced*. In case CHANSERV is having issues, that should be 'qmx' or 'ansuz'.  
  
Beyond that, remember that we are focused on keeping this software *usable*, for developers and end-users alike. To limit the amount of time we spend debugging mysterious bugs, we expect you to specify how you intend your code to deal with a range of inputs. We then expect unit tests to be included to ensure that the code satisfies its specifications. Your code is far less likely to make it into our repository if it does not include tests. If it does include tests, it will have to pass all of them before being merged.  
  
(For now,) that's all we ask!  
Happy coding!  
