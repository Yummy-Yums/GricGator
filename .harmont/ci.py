"""Harmont CI pipeline — dogfood."""
from __future__ import annotations

import harmont as hm

@hm.pipeline(
    "ci",
    env={"CI": "true"},
    default_image="ubuntu:24.04",
    triggers=[
        hm.push(branch="main"),
    ],
)
def ci() -> list:
    project = hm.rust.project(path="gricgator-app")
    return hm.group([
        project.test(flags=("--lib",), packages=("harmont-cli",)),
        project.clippy(),
        project.fmt(),
    ])
