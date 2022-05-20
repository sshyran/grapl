#!/bin/bash

set -euo pipefail

readonly NOMAD_LOGS_DEST=/tmp/nomad-agent.log
readonly CONSUL_LOGS_DEST=/tmp/consul-agent.log
THIS_DIR=$(dirname "${BASH_SOURCE[0]}")
readonly THIS_DIR

ensure_cros_bridge_networking_workaround() {
    # Suggest the Nomad bridge networking hack in building.md if module is not found
    # Due to https://github.com/hashicorp/nomad/issues/10902

    # "is this crOS?" per
    # reddit.com/r/Crostini/comments/kuhwky/how_to_test_in_shell_script_whether_inside/girzz2y/
    if [[ -f /dev/.cros_milestone ]]; then
        module_filepath="/lib/modules/$(uname -r)/modules.builtin"
        if ! grep -q "_/bridge.ko" "$module_filepath"; then
            echo "It looks like you're on ChromeOS, but haven't installed the Nomad bridge networking workaround."
            # shellcheck source-path=SCRIPTDIR
            source "${THIS_DIR}/../../etc/chromeos/lib/installs.sh"
            (
                install_nomad_chromeos_workaround
            )
            echo "ChromeOS Nomad bridge networking workaround should now be installed. Continuing..."
        fi
    fi
}

ensure_firecracker_driver_installed() {
    expected_cni_path="/opt/nomad/plugins/firecracker-task-driver"
    if [[ ! -f "${expected_cni_path}" ]]; then
        echo "It looks like you don't have the Firecracker nomad stuff set up yet."
        # shellcheck source-path=SCRIPTDIR
        source "${THIS_DIR}/../../etc/chromeos/lib/installs.sh"
        (install_nomad_firecracker)
        echo "Continuing..."
    fi
}

ensure_valid_env() {
    ensure_cros_bridge_networking_workaround
    ensure_firecracker_driver_installed

    # ensure that all dependencies are available
    if [[ -z $(command -v nomad) ]]; then
        echo "Nomad must be installed. Please follow the install instructions at https://www.nomadproject.io/downloads"
        exit 2
    fi

    if [[ -z $(command -v consul) ]]; then
        echo "Consul must be installed. Please follow the install instructions at https://www.consul.io/downloads"
        exit 2
    fi
}

create_dynamic_consul_config() {
    # clear file if it exist
    if [[ -f "${THIS_DIR}/consul-dynamic-conf.hcl" ]]; then
        rm "${THIS_DIR}/consul-dynamic-conf.hcl"
    fi

    GOSSIP_KEY=$(consul keygen)

    # generate the file
    cat << EOF > "${THIS_DIR}/consul-dynamic-conf.hcl"
encrypt = "$GOSSIP_KEY"
EOF
}

start_nomad_detach() {
    ensure_valid_env
    create_dynamic_consul_config

    echo "Starting nomad and consul locally. Logs @ ${NOMAD_LOGS_DEST} and ${CONSUL_LOGS_DEST}."
    # Consul/Nomad  will run forever until `make down` is invoked."

    # The client is set to 0.0.0.0 here so that it can be reached via pulumi in docker.
    consul agent \
        -client 0.0.0.0 -config-file "${THIS_DIR}/consul-agent-conf.hcl" \
        -config-file "${THIS_DIR}/consul-dynamic-conf.hcl" \
        -dev > "${CONSUL_LOGS_DEST}" &
    local -r consul_agent_pid="$!"

    # Ensure Consul agent is up and running
    (
        readonly attempts=15
        # shellcheck disable=SC2016
        timeout --foreground "${attempts}" bash -c -- "$(
            cat << EOF
                # General rule: Variable defined in this EOF? Use \$
                set -euo pipefail
                wait_attempt=1
                while [[ -z \$(consul info 2>&1 | grep "leader = true") ]]; do
                    if ! ps -p "${consul_agent_pid}" > /dev/null; then
                        echo "Consul Agent unexpectedly exited?"
                        exit 42
                    fi

                    echo "Waiting for consul-agent [\${wait_attempt}/${attempts}]"
                    sleep 1
                    ((wait_attempt=wait_attempt+1))
                done
                echo "consul-agent ready"
EOF
        )"
    )

    # shellcheck disable=SC2024
    sudo nomad agent \
        -config="${THIS_DIR}/nomad-agent-conf.nomad" \
        -dev-connect > "${NOMAD_LOGS_DEST}" &
    local -r nomad_agent_pid="$!"

    # Ensure Nomad agent is ready
    (
        readonly attempts=30
        # shellcheck disable=SC2016
        timeout --foreground "${attempts}" bash -c -- "$(
            cat << EOF
                # General rule: Variable defined in this EOF? Use \$
                set -euo pipefail
                wait_attempt=1
                while [[ -z \$(nomad node status 2>&1 | grep ready) ]]; do
                    if ! ps -p "${nomad_agent_pid}" > /dev/null; then
                        echo "Nomad Agent unexpectedly exited?"
                        exit 42
                    fi

                    echo "Waiting for nomad-agent [\${wait_attempt}/${attempts}]"
                    sleep 1
                    ((wait_attempt=wait_attempt+1))
                done
                echo "nomad-agent ready"
EOF
        )"
    )

    "${THIS_DIR}/nomad_run_local_infra.sh"
    echo "Deployment complete"
}

start_nomad_detach
