[private]
default:
    @just --list

serve:
    PORTFOLIO_CONFIG_PATH="./test-config" dx serve
