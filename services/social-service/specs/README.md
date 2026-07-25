# social-service Specs

`component.spec.json` declares the Social bounded-context service contract. Server bootstrap uses
the installed IM PostgreSQL authority; in-memory implementations remain explicit test fixtures.
The service owns social business behavior, while route crates own HTTP adaptation and the process
host owns listener lifecycle.
