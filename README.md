# Bhop script
Efficient and fast script for simulating key presses for bunny hopping. It runs as fast as it can when the input key is pressed and blocks when it is not.

I originally used [keyboard](https://github.com/boppreh/keyboard) with Python, but it ran too slowly. That was my motivation for this program. [rdev](https://github.com/Narsil/rdev) was used here for dealing with input, except for simulating the key press. rdev's `simulate` function was not working in games, so I dug through [keyboard's implementation](https://github.com/boppreh/keyboard/blob/master/keyboard/_winkeyboard.py) and found it was using [WIN32's `keybd_event` under the hood](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-keybd_event). Doing the same in place of `simulate` worked well.
