#!/usr/bin/env python
import XenAPI
import sys


def start_session():
    session = XenAPI.xapi_local()
    session.xenapi.login_with_password("root", "xxxx")
    return session.xenapi


if __name__ == "__main__":
    if sys.flags.interactive:
        print("Starting XenAPI session...")
        print("You can now copy/paste commands from the other script:")
        print(
            "[example]>>> sr_rec_list = [s.SR.get_record(sr_ref) for sr_ref in s.SR.get_all()]"
        )
        s = start_session()
    else:
        print("Usage: python -i repl.py")
