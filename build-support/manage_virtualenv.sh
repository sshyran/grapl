#!/usr/bin/env bash

set -euo pipefail

# Helpful constants for dealing with ANSI codes
readonly RESET="\\u001b[0m"
readonly BOLD="\\u001b[1m"
readonly UNDERLINE="\\u001b[4m"

# Additional constants
readonly python=python3
readonly virtualenv=build-support/venv
readonly pip="${virtualenv}/bin/pip"
readonly requirements_file=3rdparty/python/requirements.txt
readonly constraints_file=3rdparty/python/constraints.txt

function log() {
    echo >&2 -e "${BOLD}[${0}] $(date "+%Y-%m-%d %T"): ${UNDERLINE}${*}${RESET}"
}

# Main command function
function populate() {
    log "Generating virtual environment from ${constraints_file}"
    create_base_venv
    "${pip}" install --requirement "${constraints_file}"
    exit_message
}

# Main command function
function regenerate_constraints() {
    log "Re-generating ${constraints_file} from ${requirements_file} and Pants dependencies"
    create_base_venv
    "${pip}" install \
        --requirement "${requirements_file}" \
        --requirement <(./pants dependencies --type=3rdparty ::)
    create_constraints_file
    exit_message
}

# Remove any existing virtualenv, to ensure we're running with a
# "clean slate".
function create_base_venv() {
    rm -Rf "${virtualenv}"
    "${python}" -m venv "${virtualenv}"
    "${pip}" install pip wheel --upgrade
}

function create_constraints_file() {
    cat << EOF > "${constraints_file}"
# Generated by ${0} on $(date)
# DO NOT EDIT BY HAND
EOF

    # Apparently `pip freeze` erroneously inserts `pkg-resources==0.0.0`
    # on Ubuntu / Debian, which tends to breaks things. So we'll just
    # filter that out.
    #
    # https://stackoverflow.com/questions/39577984/what-is-pkg-resources-0-0-0-in-output-of-pip-freeze-command
    "${pip}" freeze --all |
        grep -v "pkg_resources==0.0.0" \
            >> "${constraints_file}"
}

function exit_message() {
    log
    log "A virtual environment for Grapl has been created for you at '${virtualenv}'"
    log "Please run the following to activate it:"
    log
    log "source ${virtualenv}/bin/activate"
    log
    log "Alternatively, if using 'direnv', add the following 2 lines to an '.envrc' file in the repository root:"
    echo "export VIRTUAL_ENV=${virtualenv}"
    echo "export PATH=$PATH:/${VIRTUAL_ENV}/bin"
}

function usage() {
    cat >&2 << EOF

    Usage: ${0} [populate|regenerate-constraints|help]

    Manage the Python virtual environment for Grapl.

    Arguments:

    populate:
    Create and populate a Python virtual environment containing all
    dependencies declared in ${constraints_file}.

    regenerate-constraints:
    Updates the ${constraints_file} file with all currently-declared
    3rd-party dependencies, declared in ${requirements_file} and in
    Pants configuration.

    help:
    Prints this usage message.

EOF

}

REPOSITORY_ROOT=$(git rev-parse --show-toplevel)
readonly REPOSITORY_ROOT

(
    cd "${REPOSITORY_ROOT}"
    case "${1:-help}" in
        populate)
            populate
            ;;
        regenerate-constraints)
            regenerate_constraints
            ;;
        help)
            usage
            ;;
        *)
            usage
            exit 1
            ;;
    esac
)
