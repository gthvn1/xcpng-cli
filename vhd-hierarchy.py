#!/usr/bin/env python

import os
import subprocess
from collections import defaultdict


def scan_vhd_files(directory):
    """Run `vhd-util scan` for the given VHD files and return parsed output."""
    scan_command = "vhd-util scan -f -m '{}/*.vhd'".format(directory)
    proc = subprocess.Popen(
        scan_command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )
    out, err = proc.communicate()

    if err:
        print("Scan failed: {}".format(err))
        return None

    return parse_vhd_scan_output(out)


def parse_vhd_scan_output(output):
    """Parse the output of `vhd-util scan` to extract VHD hierarchy."""
    vhd_info = defaultdict(dict)  # Store VHD info in a dict with VHD filename as key

    # line looks like: vhd=2158892d-8fc2-41de-97d3-9de93beb5d99.vhd capacity=2147483648 size=21021184 hidden=1 parent=a0199a90-dcbf-477b-8114-953950cb586c.vhd
    for line in output.strip().splitlines():
        parts = line.split()
        vhd = parts[0].split("=")[1]  # vhd filename
        parent = parts[4].split("=")[1]  # parent filename (or 'none')

        vhd_info[vhd]["parent"] = parent if parent != "none" else None

    return vhd_info


def build_vhd_hierarchy(vhd_info):
    """Build a hierarchy of VHDs based on parent-child relationships."""
    hierarchy = defaultdict(list)
    root_vhds = []

    for vhd, info in vhd_info.items():
        parent = info["parent"]
        if parent:
            hierarchy[parent].append(vhd)
        else:
            root_vhds.append(vhd)

    return root_vhds, hierarchy


def print_hierarchy(root_vhds, hierarchy, level=0):
    """Recursively print the VHD hierarchy."""
    indent = "    " * level
    for vhd in root_vhds:
        print("{}{}".format(indent, vhd))
        if vhd in hierarchy:
            print_hierarchy(hierarchy[vhd], hierarchy, level + 1)


if __name__ == "__main__":
    directory = "/var/run/sr-mount/"
    subdirs = [directory + subdir for subdir in os.listdir(directory)]

    for sdir in subdirs:
        print("Scanning {}".format(sdir))
        vhd_info = scan_vhd_files(sdir)
        if vhd_info:
            root_vhds, hierarchy = build_vhd_hierarchy(vhd_info)
            print("VHD Hierarchy Report:")
            print_hierarchy(root_vhds, hierarchy)
