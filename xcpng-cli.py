import XenAPI
import argparse
import Xcpng


def main():
    # Create an ArgumentParser object
    parser = argparse.ArgumentParser(
        description="Process some IPs, username, and password."
    )

    # Add arguments for --ips, --username, and --password
    parser.add_argument("--ips", type=str, help="Comma-separated list of IPs")
    parser.add_argument("--username", type=str, help="Username for authentication")
    parser.add_argument("--password", type=str, help="Password for authentication")

    args = parser.parse_args()

    # Split the comma-separated IP addresses into a list
    ips = args.ips.split(",") if args.ips else []

    username = args.username
    password = args.password

    if not (username and password and ips):
        print(
            "USAGE: python3 ./xcpng-cli.py --ips 1.2.3.4,1.2.3.5 --username <USER> --password <PASS>"
        )
        return

    for ip in ips:
        session = XenAPI.Session(f"https://{ip}:443", ignore_ssl=True)
        try:
            print(f"\n-- Connecting to {ip}\n")
            session.xenapi.login_with_password(
                f"{username}", f"{password}", "1.0", "testing"
            )
            vdis_ref = Xcpng.get_vdis_ref(session.xenapi)
            print("\n-- Disks --\n")
            Xcpng.vdis_info(session.xenapi, vdis_ref)
            print("\n-- SR --\n")
            Xcpng.sr_info(session.xenapi)
        finally:
            session.xenapi.session.logout()


if __name__ == "__main__":
    main()
