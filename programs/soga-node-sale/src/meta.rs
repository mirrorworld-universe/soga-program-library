#![allow(missing_docs)]

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "HyperFuse Node Sale Program",
    project_url: "https://node.sonic.game",
    contacts: "email:security@sonic.game",
    policy: "https://github.com/mirrorworld-universe/soga-program-library/blob/main/SECURITY.md",
    auditors: "Beosin"
}