#!/usr/bin/env python

import XenAPI

session = XenAPI.xapi_local()

def sr_info(s):
    """
    Display all VDIs found on each SR
    """
    sr_rec_list = [s.SR.get_record(sr_ref) for sr_ref in s.SR.get_all()]
    # Only keep sr with VDIs
    sr_rec_filtered = [sr for sr in sr_rec_list if sr['VDIs']]

    for sr_rec in sr_rec_filtered:
        print("> Looking VDI for SR {} {}".format(sr_rec['uuid'], sr_rec['name_label']))
        # sr_rec['VDIs'] contains a list of VDI ref
        vdi_rec_list = [s.VDI.get_record(vdi_ref) for vdi_ref in sr_rec['VDIs']]
        for vdi_rec in vdi_rec_list:
            print("    > VDI {} {}".format(vdi_rec['name_label'], vdi_rec['uuid']))
            print("        -> type: {}".format(vdi_rec['type']))
            if 'vhd-parent' in vdi_rec['sm_config']:
                print("        -> parent: {}".format(vdi_rec['sm_config']['vhd-parent']))
            if vdi_rec['is_a_snapshot']:
                print("        -> is a snaphost")
            else:
                print("        -> is a not snaphost")

def vdis_info(s, vdis_ref):
    """
    Display information about VDIs found in the list of vdis_ref
    """
    for vdi_ref in vdis_ref:
        vdi_uuid = s.VDI.get_uuid(vdi_ref)
        print("-> [{}]".format(vdi_uuid))

        vdi_name = s.VDI.get_name_label(vdi_ref)
        vdi_location = s.VDI.get_location(vdi_ref)
        vdi_snap = s.VDI.get_snapshot_of(vdi_ref)
        vdi_parent = s.VDI.get_parent(vdi_ref)

        print("  -> [NAME       ] {}".format(vdi_name))
        print("  -> [LOCATION   ] {}".format(vdi_location))
        print("  -> [SNAPSHOT OF] {}".format(vdi_snap))
        print("  -> [PARENT     ] {}".format(vdi_parent))

def get_vdis_ref(s):
    """
    Get all VDIs that are used in the pool. It first find hosts and then it
    goes through them to find resident VMs and their VDI.
    It returns a list of VDIs Refs.
    """
    vdis_found = []

    # Get the list of all host
    host_ref_list = s.host.get_all()
    for host_ref in host_ref_list:
        host_name = s.host.get_name_label(host_ref)
        resident_vms = s.host.get_resident_VMs(host_ref)
        print("[Host {}]".format(host_name))

        for vm_ref in resident_vms:
            vm_name = s.VM.get_name_label(vm_ref)
            vm_uuid = s.VM.get_uuid(vm_ref)
            print("  -> [VM] {} - {}".format(vm_name, vm_uuid))

            vbds = s.VM.get_VBDs(vm_ref)
            for vbd_ref in vbds:
                vdi_ref = s.VBD.get_VDI(vbd_ref)
                if vdi_ref == 'OpaqueRef:NULL':
                    vbd_uuid = s.VBD.get_uuid(vbd_ref)
                    print("    -> [VBD] {} ...has no VDI".format(vbd_uuid))
                    print("             Probably not in database...")
                else:
                    vdis_found.append(vdi_ref)
                    vdi_uuid = s.VDI.get_uuid(vdi_ref)
                    vdi_name = s.VDI.get_name_label(vdi_ref)
                    print("    -> [VDI] {}/({})".format(vdi_uuid, vdi_name))

    return vdis_found

if __name__ == '__main__':
    try:
        print("-- GENERAL INFOS --\n")
        session.xenapi.login_with_password("root", "xxxx")
        vdis_ref = get_vdis_ref(session.xenapi)
        print("\n-- DISKS INFO --\n")
        vdis_info(session.xenapi, vdis_ref)
        print("\n-- SR INFO --\n")
        sr_info(session.xenapi)

    finally:
        session.xenapi.session.logout()

