# comms-social-service-bin Specs

`component.spec.json` declares the standalone Social service process boundary. The host installs the
shared IM PostgreSQL pool, builds the Social service through its public bootstrap, and binds only
after required state is available. File and memory authority are test fixtures, not server modes.
