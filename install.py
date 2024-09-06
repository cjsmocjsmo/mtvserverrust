import argparse
import os
import subprocess

CWD = os.getcwd()

class MTVChecks:
    def __init__(self) -> None:
        self.mtv_dir = "/usr/share/MTV"
        self.thumbnails_dir = "/usr/share/MTV/thumbnails"
        self.static_dir = "/usr/share/MTV/static"
        self.db_file = "/usr/share/MTV/mtv.db"
        self.service_file = "/etc/systemd/system/mtvserverrust.service"
        self.service_file_loc = f"{CWD}/mtvserverrust.service"

    def check_mtv_dir(self):
        if os.path.exists(self.mtv_dir):
            return True
        else:
            return False
    def create_mtv_dir(self):
        if not os.path.exists(self.mtv_dir):
            subprocess.run(["sudo", "mkdir", self.mtv_dir])
            subprocess.run(["sudo", "chown", "-R", "pimedia:pimedia", self.mtv_dir])
            subprocess.run(["sudo", "chmod", "-R", "775", self.mtv_dir])

    def create_thumbnails_dirs(self):
        if not os.path.exists(self.thumbnails_dir):
            subprocess.run(["sudo", "mkdir", self.thumbnails_dir])
            subprocess.run(["sudo", "chown", "-R", "pimedia:pimedia", self.thumbnails_dir])
            subprocess.run(["sudo", "chmod", "-R", "775", self.thumbnails_dir])

    def create_static_dir(self):
        if not os.path.exists(self.static_dir):
            subprocess.run(["sudo", "mkdir", self.static_dir])
            subprocess.run(["sudo", "chown", "-R", "pimedia:pimedia", self.static_dir])
            subprocess.run(["sudo", "chmod", "-R", "775", self.static_dir])

    def create_db_file(self):
        if not os.path.exists(self.db_file):
            subprocess.run(["sudo", "touch", self.db_file])

class MTVBuild:
    def __init__(self) -> None:
        self.mtv_dir = "/usr/share/MTV"
        self.mtv_loc = "/usr/bin/"
        self.thumbnails_dir = "/usr/share/MTV/thumbnails"
        self.static_dir = "/usr/share/MTV/static"
        self.db_file = "/usr/share/MTV/mtv.db"
        self.service_file = "/etc/systemd/system/mtvserverrust.service"
        self.service_file_loc = f"{CWD}/mtvserverrust.service"

    def build_mtv(self):
        subprocess.run(["cargo", "build", "--release"])
        subprocess.run(["sudo", "cp", f"{CWD}/target/release/mtvserverrust", self.mtv_loc])
        new_mtv_loc = f"{self.mtv_loc}/mtvserverrust"
        subprocess.run(["sudo", "chmod", "775", new_mtv_loc])
        subprocess.run(["sudo", "cp", self.service_file_loc, self.service_file])

    def start_mtv(self):
        subprocess.run(["sudo", "systemctl", "start", "mtvserverrust.service"])

    def enable_mtv(self):
        subprocess.run(["sudo", "systemctl", "enable", "mtvserverrust.service"])        

    def stop_mtv(self):
        subprocess.run(["sudo", "systemctl", "stop", "mtvserverrust.service"])
    
    def disable_mtv(self):
        subprocess.run(["sudo", "systemctl", "disable", "mtvserverrust.service"])

def main():
    parser = argparse.ArgumentParser(description="CLI for Rusic music server.")
    parser.add_argument("version", type=str, help="Version of the software")
    parser.add_argument("-i", "--install", action="store_true", help="Install the program")
    parser.add_argument("-u", "--update", action="store_true", help="Update the program")
    parser.add_argument("-d", "--delete", action="store_true", help="Delete the program")

    args = parser.parse_args()

    if args.install:
        checks = MTVChecks()
        checks.create_mtv_dir()
        checks.create_thumbnails_dirs()
        checks.create_static_dir()
        checks.create_db_file()

        build = MTVBuild()
        build.build_mtv()
        build.start_mtv()
        build.enable_mtv()

    if args.update:
        build = MTVBuild()
        build.stop_mtv()
        build.disable_mtv()
        build.build_mtv()
        build.start_mtv()
        build.enable_mtv()

    if args.delete:
        build = MTVBuild()
        build.stop_mtv()
        build.disable_mtv()
        subprocess.run(["sudo", "rm", "-rf", "/usr/share/MTV"])
        subprocess.run(["sudo", "rm", "/etc/systemd/system/mtvserverrust.service"])
        subprocess.run(["sudo", "rm", "/usr/bin/mtvserverrust"])

if __name__ == "__main__":
    main()