# A source is asked for a named account, and sources are not interchangeable

Status: accepted (2026-08-09, design session on the crate discarding every answer it asks for)

The crate asked the GitHub command-line tool four questions and acted on none of the answers. A
presence check returned a `bool` that was dropped. `gh auth switch --user` returned an exit status
that was dropped, so an account the tool does not hold was accepted and the token that followed
belonged to whichever account was already active. `gh auth token` returned an exit status that was
dropped and output that was asserted to be UTF-8, so a logged-out machine produced an empty
credential that failed later at the API boundary, where the cause was no longer visible. Those are
one defect rather than four: authentication was modelled as fire-and-forget subprocesses whose
results are advisory, behind a type that cannot represent the states those subprocesses produce.

**A credential is obtained for a named account, and obtaining it changes nothing.** `gh auth token
--user <account>` names the account per request, so the machine's active account is never moved
and nothing is left to restore. Measured 2026-08-09: an account the tool does not hold exits 1 and
prints nothing, so the account is refused rather than silently substituted. Switching leaves the
crate entirely rather than becoming optional — an optional mutation is one some caller selects,
and none wants it.

**The promise is a credential to act as an account, not merely to read as one.** That is what
makes sources answer different questions rather than one question in different ways: an
installation token acts as the application it belongs to, with that application's permissions and
authorship, so it cannot stand in for a person's login even where a read would be
indistinguishable. Each source is therefore its own function, with its own selector, its own token
and its own failures, named for the source it is; a further source lands beside it rather than
beneath a shared trait. The crate is a home for credential sources, not an abstraction over them.

**Two failures are named and the rest is opaque.** A caller branches on exactly two: the tool is
absent from the machine, and the tool does not hold the account. Each closes by a distinct act a
person performs — install the tool, or log in as that account. Every other cause differs in origin
but not in what anyone does about it, so naming more manufactures branches nothing takes. Absence
is learned from the same call rather than a separate one: the spawn failing to find the program is
the answer, which is what stops it from being discarded the way a separate probe's `bool` was.

## Considered options

- **A trait over credential sources.** The sources disagree on what selects a credential, on how
  long one lives and on how obtaining one fails, so a trait spanning them is the intersection of
  three disagreements — which is what the discarded `Authentication` trait already was, and why it
  could express neither a failure nor an account it had not been handed. It also has no caller: a
  single source is named directly, and code generic over one implementation is a vocabulary rather
  than an abstraction.
- **A variant for every reachable state.** A non-zero exit for an unanticipated reason, output that
  is not UTF-8, and output that is empty are distinguishable at the point they occur and identical
  in what anyone does about them.
- **Retaining the mutating call behind an argument.** Preserves the hazard for whoever passes the
  argument, and leaves every caller with a global side effect to reason about.
- **Folding the crate into its one live consumer.** The crate stays its own repository because it
  is where later sources belong; a source obtained without the command-line tool has no home in a
  program that reads configurations.

## Consequences

- **The `Authentication` trait and `GitHubCliAuthentication` are removed**, which is a breaking
  change. Dependents keep building against the revisions their lockfiles already name; two are
  deliberately left there, so the break reaches them as a compile error whenever they update
  rather than as a silent change of behaviour.
- **The subprocess is injected behind a private trait with one production implementation**, so a
  test drives chosen exit codes and chosen bytes, and asserts that the arguments name the account
  and never switch to it. The seam is over a thing the crate does not own, is never visible in a
  caller's types, and exists because the test is a caller of it.
- **The public error type is concrete rather than erased.** A library returning an opaque error
  chain makes its own states recoverable only by downcasting, which is the defect one layer up.
- **A token is a transparent wrapper over a secret string that cannot be constructed empty**, so a
  call that succeeds while producing nothing is unrepresentable rather than checked for.
- **A token carries no expiry.** An optional one would admit two states nothing produces — a
  long-lived credential dated, and a short-lived one undated — and a mandatory one would require a
  sentinel that asserts a falsehood as a fact.
- **Continuous integration no longer receives a token.** Nothing under test reads one, and the
  grant existed only so that the single live-machine test could find a credential to assert on.
