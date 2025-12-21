# TODO

* [X] Open the current note in neovim.
  * [X] Add a check if the file exists, don't overwrite it
    - This allows that we don't accidently delete our note.
  * ~[X] Check if using pathbuf have any use for us, or it's just another dependency.~. We don't need it
  because we don't use any features which are exposed by pathbuf
* [ ] Clean up chrono crate, minimize dependencies
