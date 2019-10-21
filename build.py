#!/usr/bin/env python3

import argparse
import os
import sys


PROJECT_ROOT = sys.path[0]

AUDITORS_DIR = os.path.join(PROJECT_ROOT, "auditors")
BIN_DIR = os.path.join(PROJECT_ROOT, "bin")
BUILD_DIR = os.path.join(PROJECT_ROOT, "build")
OUT_DIR = os.path.join(PROJECT_ROOT, "out")
SCRIPT_DIR = os.path.join(PROJECT_ROOT, "cmake", "scripts")

FNS_WORKER_DIR = os.path.join(
    PROJECT_ROOT, "mesatee_services", "fns", "sgx_trusted_lib", "src",
    "trusted_worker")
WORKER_USED = os.path.join(FNS_WORKER_DIR, "yourworker")

FNS_LIB_TARGET = "sgxlib-fns"
FNS_SIGNED_ENCLAVE = f"{BIN_DIR}/fns.enclave.signed.so"
FNS_ENCLAVE_INFO = f"{OUT_DIR}/fns_enclave_info.txt"


def build_fns_worker(worker):
    src_dir = os.path.join(FNS_WORKER_DIR, worker)
    if not os.path.exists(src_dir):
        print(f"skipping {worker} due to missing sources")
        return

    remove_rf(WORKER_USED)
    os.symlink(src_dir, WORKER_USED)
    os.system(f"cd {BUILD_DIR} && make {FNS_LIB_TARGET}")
    remove_rf(WORKER_USED)

    os.system(f'sed -i "s/fns/{worker}_fns/" {FNS_ENCLAVE_INFO}')
    os.rename(FNS_ENCLAVE_INFO, f"{OUT_DIR}/{worker}_fns_enclave_info.txt")
    os.rename(FNS_SIGNED_ENCLAVE, f"{BIN_DIR}/{worker}_fns.enclave.signed.so")


def build_mesatee():
    worker = "hello"

    src_dir = os.path.join(FNS_WORKER_DIR, worker)
    if not os.path.exists(src_dir):
        print(f"skipping {worker} due to missing sources")
        return

    remove_rf(WORKER_USED)
    os.symlink(src_dir, WORKER_USED)
    os.system(f"cd {BUILD_DIR} && make")
    remove_rf(WORKER_USED)


def do_clean():
    if not os.path.exists(BUILD_DIR):
        print("project is clean")
        return

    os.system(f"cd {BUILD_DIR} && make clean && cd .. && rm -rf {BUILD_DIR}")


def gen_enclave_sig():
    os.system(f"{SCRIPT_DIR}/gen_enclave_sig.sh")


def parse_args():
    parser = argparse.ArgumentParser("mesatee-build")

    parser.add_argument("--fresh", action="store_true",
                        help="whether to build from scratch")
    parser.add_argument("--sign", action="store_true",
                        help="whether to renew sig for enclave_info.txt")
    parser.add_argument("--workers", nargs="+", help="worker to build")
    parser.add_argument("--clean", action="store_true",
                        help="clean the project")

    return parser.parse_args()


def prepare_fresh_build():
    remove_rf(BUILD_DIR)
    os.makedirs(BUILD_DIR)
    os.system(f"cd {BUILD_DIR} && SGX_MODE=SW cmake ..")


def remove_rf(path):
    os.system(f"rm -rf {path}")


# set env
os.putenv("MESATEE_AUDITORS_DIR", AUDITORS_DIR)
os.putenv("MESATEE_OUT_DIR", OUT_DIR)
os.putenv("MESATEE_PROJECT_ROOT", PROJECT_ROOT)

args = parse_args()

if args.fresh or not os.path.exists(BUILD_DIR):
    prepare_fresh_build()

if args.workers is not None:
    for w in args.workers:
        print(f"building {w} ...")
        build_fns_worker(w)
        print(f"done building {w} ...")

if args.fresh:
    print("building the whole project ...")
    build_mesatee()
    print("done building the whole project ...")

if args.sign:
    print("renewing the sig for enclave_info.txt ...")
    gen_enclave_sig()
    print("done renewing the sig for enclave_info.txt ...")

if args.clean:
    print("cleaning project ...")
    do_clean()
    print("done cleaning project")
