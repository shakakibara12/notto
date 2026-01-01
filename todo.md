# TODO

* [X] Open the current note in neovim.
  * [X] Add a check if the file exists, don't overwrite it
    - This allows that we don't accidently delete our note.
  * ~[X] Check if using pathbuf have any use for us, or it's just another dependency.~. We don't need it
  because we don't use any features which are exposed by pathbuf
* ~[ ] Clean up chrono crate, minimize dependencies~ Doesn't matter, rust only compiles what is needed.
* [ ] Give the user a option to create note for the next day.
  - Sometimes when i exec notto to write something and there is already the previous day's note available, it will just open that instead of creating a new one.
  - This happens because if i edit a file after 12 am let's say, the file will be created with tomorrow's timestamp, which makes sense. But that prohibits us from creating another note with today's date (because it's already taken)
  - Add a cmdline option that allows us to create tomorrow's journal (cli or any other choice.)
