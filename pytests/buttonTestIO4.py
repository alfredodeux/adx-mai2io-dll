# Install python3 HID package https://pypi.org/project/hid/
# read from buttons with IO4 firmware

import hid
from timeit import default_timer as timer

USB_VID = 0x0CA3

print("Opening HID device with VID = 0x%X" % USB_VID)

for dict in hid.enumerate(USB_VID):
    print(dict)
    dev = hid.Device(dict['vendor_id'], dict['product_id'])
    if dev:
        while True:
            start = Timer()
            x = dev.read(31)
            end = Timer()
            print(x)
            # print(end-start)
