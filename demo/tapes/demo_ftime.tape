Require bash
Require jq
Require ftime

Set Shell "bash"
Set FontSize 30
Set Width 1100
Set Height 720
Set Padding 24
Set TypingSpeed 45ms
Set Framerate 30

Hide
Type "cd $FTIME_DEMO_DIR"
Enter
Sleep 700ms
Show

Type "ftime"
Enter
Sleep 2200ms

Hide
Type "clear"
Enter
Sleep 400ms
Show

Type "ftime -a"
Enter
Sleep 2200ms

Hide
Type "clear"
Enter
Sleep 400ms
Show

Type "ftime --json | jq -r '.path'"
Enter
Sleep 1800ms
