#set debug remote 1
set remotetimeout unlimited
target remote | openocd
load
continue
