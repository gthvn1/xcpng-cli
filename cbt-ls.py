#!/usr/bin/env python3

import argparse

from bitstring import BitStream
import base64

import XenAPI


session = XenAPI.xapi_local()
sx = session.xenapi

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Get a list of the blocks that have changed between two VDIs")
    
    parser.add_argument("vdi_from", type=str, help="The earlier VDI UUID snapshot")
    parser.add_argument("vdi_to", type=str, help="The later VDI snapshot. This VDI cannot be attached to a VM at the time this comparison is made")
    args = parser.parse_args()
    
    try:
        sx.login_with_password("root", "xxxx")
        from_ref = sx.VDI.get_by_uuid(args.vdi_from)
        to_ref = sx.VDI.get_by_uuid(args.vdi_to)
        bitmap = sx.VDI.list_changed_blocks(from_ref, to_ref)
        data = BitStream(bytes=base64.b64decode(bitmap))
        for idx, bit in enumerate(data):
            if bit == 1:
                print(f"Bit set at index: {idx}")
    finally:
        sx.session.logout()
