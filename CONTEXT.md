# github_authentication

Obtains a credential to act as a named GitHub account, from whichever source holds one. A
source is asked and answers; nothing about the machine changes as a result.

## Language

**Credential source**:
A means of obtaining a credential for a GitHub account — the command-line tool today, a minted
installation token later. Sources are peers rather than substitutes: each is asked in its own
terms and answers with its own failures.
_Avoid_: provider, backend, method, strategy

**GitHub account**:
The account a credential acts as, named on each request. Not the owner of a repository, which
is an address rather than something anything acts as.
_Avoid_: identity, username, login, user

**Act as**:
What a credential permits: to be GitHub's answer to who is doing this, covering both what the
doer may see and the authorship it leaves behind. Narrower than being able to read something,
which two different accounts can do indistinguishably.
_Avoid_: authenticate as, authorise, impersonate

**Token**:
The credential a source returns, valid to act as the account it was asked for. Carries no
expiry: a credential that must be re-obtained before use is a different thing, not a token with
a date on it.
_Avoid_: secret, key, PAT, bearer

**Unheld**:
Describes an account a source cannot answer for, distinct from a source absent from the machine
altogether. Both are refusals a person closes, by different acts.
_Avoid_: unknown, invalid, missing
