
Before building, need to install libusb development files ...
On my Fedora system, this is package "libusb-devel"
    -- PKG_CONFIG

libusb appread in here
ls /usr/lib64/pkgconfig/

-- To grant permissions to te device

https://unix.stackexchange.com/questions/44308/understanding-udev-rules-and-permissions-in-libusb


/lib/udev/rules.d/50-udev-default.rules

# 'libusb' device nodes
SUBSYSTEM=="usb", ENV{DEVTYPE}=="usb_device", MODE="0666"

It is possible to be more specific with the permissions, you can specify a device by things like USB vendor (VID) and product ID (PID). So it would be possible to make a file which would work for just your scanner, and leave all other USB devices with the default permissions. There's a page about Udev at https://wiki.archlinux.org/index.php/udev which looks useful for all this. If you make a file which starts with a higher number, like "60-my-usb-scanner.rules" (higher numbers take priority) and have a rule which lists you scanner like:
SUBSYSTEM=="usb", ATTRS{idVendor}=="F1E2", ATTRS{idProduct}=="1F2E", MODE="0666"

Put in a file called "60-my-usb-pm4.rules"
My PM4 is 17a4:0002 .. so 
SUBSYSTEM=="usb", ATTRS{idVendor}=="17A4", ATTRS{idProduct}=="0002", MODE="0666"

To re-load the rules
udevadm control --reload-rules