# How to Build

## Prerequisite

We only support Linux environment for now. Ubuntu 16.04/18.04 are prefered.  If
you want to build in native environment, please have nightly Rust and the
latest [Intel SGX SDK](https://01.org/intel-software-guard-extensions/downloads) installed.
As of writing, the latest Intel SGX SDK version is
[2.6.100](https://download.01.org/intel-sgx/linux-2.6/ubuntu18.04-server/), and
our testing is based on that. Detailed instructions can be found in our
[Dockerfile](https://github.com/mesalock-linux/mesatee/blob/master/Dockerfile).
Or, you can directly build in the [docker image](https://hub.docker.com/r/mesalocklinux/build-mesatee).
Please refer to our [CI configuration](../.drone.yml) for such examples.

In order to fulfill the requirements of SGX remote attestation, you need to
first log in to the brand new [Intel Trusted API Center](https://api.portal.trustedservices.intel.com/) and then subscribe to the **Linkable** version of [Intel SGX Attestation Service Utilizing Enhanced Privacy ID (EPID)](https://api.portal.trustedservices.intel.com/EPID-attestation). Then please visit the [Intel Trusted Service API portal](https://api.portal.trustedservices.intel.com/developer) and check the [Manage Subscriptions](https://api.portal.trustedservices.intel.com/developer) page for **SPID**, **Primary Key** and **Secondary Key**.

Attention: MesaTEE has switched to [IAS API v5](https://api.trustedservices.intel.com/documents/sgx-attestation-api-spec.pdf).

All of the prerequisites are:

1. Enclave signing key pair, and the the corresponding MRSIGNER value
2. SPID, and either "Primary Key", or "Secondary Key"
3. Intel IAS [Report Signing Public Key](http://software.intel.com/sites/default/files/managed/7b/de/RK_PUB.zip)

You need to also self-issue a CA certificate so that you can later issue client
certificates. Only clients with certificates issued by you can successfully
communicate with your MesaTEE deployment.

Examples of the above materials can be found at [here](../cert) (we only
included those that we are comfortable to open source though). Among them,
MRSIGNER and IAS report signing key, and the self-issued CA certificate should
be put in the ``ra_config`` section of [build.toml](../build.toml); the issued
client certificate should be assigned to the ``client_config`` section of
[build.toml](../build.toml); and the IAS SPID and Key should be configured in
the ``ias_client_config`` section of [config.toml](../config.toml).

Moreover, as we described in [Mutual Attestation: Why and
How](mutual_attestation.md), auditors' credentials should be configured in the
``audited_enclave_config`` section of [build.toml](../build.toml).

## Build Modes

After fulfill all requirements specified in [build.toml](../build.toml), you
can start to build by using either ``cmake`` or ``make``.

### The ``cmake`` Way

```
cd <MESATEE_PROJECT_ROOT>
mkdir <BUILD_DIR> && cd <BUILD_DIR>
cmake -DSGX_SDK=<SGX_SDK_PATH> -DSGX_MODE=HW .. # build with release mode
make VERBOSE=1 # enable verbose build output
```

The cmake options to change build modes:  
`-DSGX_MODE=<HW|SW>` build in hardware SGX mode or simulation SGX mode  
`-DCMAKE_BUILD_TYPE=<DEBUG|RELEASE>` build in debug/release mode  
`-DCMAKE_BUILD_TYPE=DEBUG -DCOV=1` debug with gcov enabled

### The ``make`` Way

```
. ./environment # unlike cmake, environment variables need to be sourced for makefile
make # build with release mode, or
make DBG=1 # build with debug mode, or
make DBG=1 COV=1 # debug with gcov enabled, or
make VERBOSE=1 # enable verbose build output
```

## Enabling Simulation Mode

By default, the outcome is targeting a platform with SGX hardware.  In order to
switch to SGX simulation target, please set ```-DSGX_MODE=SW``` when running ```cmake```, or ```export SGX_MODE=SW``` for ```make```

## Other Environment Variables

For ```make```, sourcing environment variables from [environment](../environment) is required; for ```cmake```, all needed environment variables are generated and configured in <BUILD_DIR>/environment.
When manually running the executables, sourcing the corresponding environment script can
help set the variables. Below is the description for the environment variables:

* ``SGX_MODE``: whether to run with hardware SGX (``HW``) or in simulation (``SW``)
* ``SGX_SDK``: path to the Intel SGX SDK
* ``MESATEE_PROJECT_ROOT``: MesaTEE project root directory
* ``MESATEE_CFG_DIR``: directory containing the runtime config
* ``MESATEE_BUILD_CFG_DIR``: directory containing the compile time config
* ``MESATEE_BIN_DIR``: directory that you want to place the generated binaries
* ``MESATEE_AUDITORS_DIR``: directory containing auditors' public keys and endorsement to TEE enclaves (digital signatures)
* ``MESATEE_TEST_MODE``: whether executing in testing mode
* ``RUST_LOG``: logging levels
* ``RUST_BACKTRACE``: whether to enable backtrace logging on crash
