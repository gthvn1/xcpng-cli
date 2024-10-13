# xcpng-cli

- For now, it is just some simple Python scripts to retrieve information mainly
about the disks, **but** eventually, we aim to build something more interactive, like the [Ranger file manager](https://github.com/ranger/ranger).
  - *xcpng-cli.py*: dumps some info about VM, SR and VDI
  - *repl.py*: runs in interactive mode and just setup a xapi session
  - *vhd-hierarchy.py*: dumps hierarchy of VDIs under `/var/run/sr-mount/`
- XenAPI has been taken from [xapi-project](https://github.com/xapi-project/xen-api/tree/7670247ae6a3c656ae60123dc550a9eb415d5ba4/python3/examples/XenAPI).
- It has been tested using python3.11
- And it was **tested a little little bit** on xcpng 8.3

- **rustyxe** is an attempt to write the Xen API command interface in Rust. This is the very beginning. We are able to send the login XML-RPC and received
the credentials.
  - `RUST_LOG=debug cargo run`
  - Todo
    - [x] get credentials
    - [ ] get information about hosts
    - [ ] get more info
    - [ ] use some curses to be able to display and navigate through information

## Run on your machine

### xcpng-ls.py
- This one can be run on your machine and it will connect to XCP-ng hosts to get information
- Usage: `python3 ./xcpng-cli.py --ips 1.2.3.4,1.2.3.5 --username <USER> --password <PASS>` 
- Here is the kind of output you can have currently:
```sh
# ./xcpng-ls.py
-- GENERAL INFOS --

[Host xcp-ng-canaweed2]
  -> [VM] Control domain on host: xcp-ng-canaweed2 - 6d10a779-f231-4b62-80d8-f594d29acbec
[Host xcp-ng-canaweed]
  -> [VM] Alpine minimal for CI - 0fcea045-c37d-542a-4340-017f9621bf9f
    -> [VBD] f9051c69-b3f1-afb3-e44c-f0b48c6f33ba ...has no VDI
             Probably not in database...
    -> [VDI] 48f3e4ed-05b6-4c26-a600-35346eedb486/(Other install media_ovebu)
  -> [VM] Control domain on host: xcp-ng-canaweed - 835f8abe-a3a5-40b0-b88a-378c96fb7f35

-- DISKS INFO --

-> [48f3e4ed-05b6-4c26-a600-35346eedb486]
  -> [NAME       ] Other install media_ovebu
  -> [LOCATION   ] 48f3e4ed-05b6-4c26-a600-35346eedb486
  -> [SNAPSHOT OF] OpaqueRef:NULL
  -> [PARENT     ] OpaqueRef:NULL

-- SR INFO --

> Looking VDI for SR 80439314-ab60-b90d-facb-530f85f218f4 DVD drives
    > VDI SCSI 1:0:1:0 5fff63ac-f4d4-4a6b-899b-ec02bd672d7b
        -> type: user
        -> is a not snaphost
> Looking VDI for SR 7d398363-7c6f-201b-9dee-18eb72b3b598 DVD drives
    > VDI SCSI 1:0:1:0 995614f0-c7fc-4cf9-abc8-5456cbddf2ed
        -> type: user
        -> is a not snaphost
> Looking VDI for SR 5d9edbbb-8c18-4093-5562-7856dc3aa62d XCP-ng Tools
    > VDI guest-tools.iso 636ec147-1a06-4cc8-96d4-6d1b8202650e
        -> type: user
        -> is a not snaphost
> Looking VDI for SR 4e8be08e-c205-3173-94b9-9fba52e6f0f5 Local storage
    > VDI Other install media_ovebu 87c54ab2-18aa-4768-b606-2b4e76bc04a9
        -> type: user
        -> parent: 2158892d-8fc2-41de-97d3-9de93beb5d99
        -> is a snaphost
    > VDI base copy 2158892d-8fc2-41de-97d3-9de93beb5d99
        -> type: user
        -> parent: a0199a90-dcbf-477b-8114-953950cb586c
        -> is a not snaphost
    > VDI Other install media_ovebu c4ab1da2-3731-405f-9214-5f18eb578065
        -> type: user
        -> parent: a0199a90-dcbf-477b-8114-953950cb586c
        -> is a snaphost
    > VDI base copy a0199a90-dcbf-477b-8114-953950cb586c
        -> type: user
        -> is a not snaphost
    > VDI Other install media_ovebu 48f3e4ed-05b6-4c26-a600-35346eedb486
        -> type: user
        -> parent: 2158892d-8fc2-41de-97d3-9de93beb5d99
        -> is a not snaphost
```

## Run on the XCP-ng host

- The two following scripts must be copied on your XCP-ng hosts.

### repl.py
- You can use `repl.py` in interactive mode. Then you have a session open and can run command easily:
```sh
# python -i repl.py
Starting XenAPI session...
You can now copy/paste commands from the other script:
[example]>>> sr_rec_list = [s.SR.get_record(sr_ref) for sr_ref in s.SR.get_all()]
>>> sr_rec_list = [s.SR.get_record(sr_ref) for sr_ref in s.SR.get_all()]
>>> sr_rec_list
[{'sm_config': {'type': 'cd'}, 'PBDs': ['OpaqueRef:02a60b50-2465-84c3-bbde-7be1d7d1da6d'], 'current_operations': {}, 'uuid': '80439314-ab60-b90d-facb-530f85f218f4', 'VDIs': ['OpaqueRef:11c310ad-3059-8b87-890f-3032066c6bf7'], 'tags': [], 'physical_size': '1073741312', 'type': 'udev', 'other_config': {'i18n-original-value-name_label': 'DVD drives', 'i18n-key': 'local-hotplug-cd', 'i18n-original-value-name_description': 'Physical DVD drives'}, 'name_label': 'DVD drives', 'allowed_operations': ['vdi_introduce', 'unplug', 'plug', 'pbd_create', 'update', 'pbd_destroy', 'vdi_clone', 'scan'],
...
```

### vhd-hierarchy.py

- It displays the hierarchy for VHD files found in `/var/run/sr-mount/`
- Here is an example of output:
```sh
Scanning /var/run/sr-mount/47ccce60-cac2-81c9-398d-4f7c8be07fa8
Scanning /var/run/sr-mount/4e8be08e-c205-3173-94b9-9fba52e6f0f5
VHD Hierarchy Report:
a0199a90-dcbf-477b-8114-953950cb586c.vhd
    c4ab1da2-3731-405f-9214-5f18eb578065.vhd
    2158892d-8fc2-41de-97d3-9de93beb5d99.vhd
        48f3e4ed-05b6-4c26-a600-35346eedb486.vhd
        87c54ab2-18aa-4768-b606-2b4e76bc04a9.vhd
```
