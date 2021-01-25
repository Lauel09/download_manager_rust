# Download_manager_rust
A downloader manager written in Rust
I had been working on it for last more than 5 days,(Code is a mess),
I just wrote whatever suited me possible that's why I had to refactor
it completely more than 2 times.
I wanted to see how flexible I could be while writing code related to 
get and post request and yup it is very difficult,Plus the fact when 
your project goes beyond 250+ lines it becomes very hard to manage 
stuffs.I can't imagine how Linux kernel and other projects with 
millions of LOC are managed.
It was fun writing this project though I had to fight a lot with the 
borrow checker and I also got introduced to Cell,Ref,RefCell and 
shared references but I avoided using them,because I wanted to keep
the project as simple as possible.
#Libraries Used:-
Only library used here is Reqwest with features = ["blocking"] and 
nothing else. It was wonderful to see that how vast Rust standard 
library is how useful it could be.
If I had been writing it in some other language like C++,I  would 
have been using a lots of external library.Yes it could have been 
much easier to write this in some language like Python or JS, but 
I wanted to see how a "low-level" language like Rust would perform 
with requests and stuffs.
Never use it,except you want to be fascinated by looking at a horrible
code that I have just brought together to "just work".
